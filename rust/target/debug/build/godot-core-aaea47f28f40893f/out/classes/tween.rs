#![doc = "Sidecar module for class [`Tween`][crate::classes::Tween].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Tween` enums](https://docs.godotengine.org/en/stable/classes/class_tween.html#enumerations).\n\n"]
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
    #[doc = "Godot class `Tween`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`tween`][crate::classes::tween]: sidecar module with related enum/flag types\n* [`ITween`][crate::classes::ITween]: virtual methods\n* [`SignalsOfTween`][crate::classes::tween::SignalsOfTween]: signal collection\n\n\nSee also [Godot docs for `Tween`](https://docs.godotengine.org/en/stable/classes/class_tween.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<Tween>` instances via Godot APIs.\n# Godot docs\nTweens are mostly useful for animations requiring a numerical property to be interpolated over a range of values. The name _tween_ comes from _in-betweening_, an animation technique where you specify _keyframes_ and the computer interpolates the frames that appear between them. Animating something with a `Tween` is called tweening.\n\n`Tween` is more suited than [`AnimationPlayer`][crate::classes::AnimationPlayer] for animations where you don't know the final values in advance. For example, interpolating a dynamically-chosen camera zoom value is best done with a `Tween`; it would be difficult to do the same thing with an [`AnimationPlayer`][crate::classes::AnimationPlayer] node. Tweens are also more light-weight than [`AnimationPlayer`][crate::classes::AnimationPlayer], so they are very much suited for simple animations or general tasks that don't require visual tweaking provided by the editor. They can be used in a \"fire-and-forget\" manner for some logic that normally would be done by code. You can e.g. make something shoot periodically by using a looped [`CallbackTweener`][crate::classes::CallbackTweener] with a delay.\n\nA `Tween` can be created by using either [`create_tween`][`crate::classes::SceneTree::create_tween`] or [`create_tween`][`crate::classes::Node::create_tween`]. `Tween`s created manually (i.e. by using `Tween.new()`) are invalid and can't be used for tweening values.\n\nA tween animation is created by adding [`Tweener`][crate::classes::Tweener]s to the `Tween` object, using [`tween_property`][`crate::classes::Tween::tween_property`], [`tween_interval`][`crate::classes::Tween::tween_interval`], [`tween_callback`][`crate::classes::Tween::tween_callback`] or [`tween_method`][`crate::classes::Tween::tween_method`]:\n\n\n```gdscript\nvar tween = get_tree().create_tween()\ntween.tween_property($Sprite, \"modulate\", Color.RED, 1.0)\ntween.tween_property($Sprite, \"scale\", Vector2(), 1.0)\ntween.tween_callback($Sprite.queue_free)\n```\n\n\nThis sequence will make the `$Sprite` node turn red, then shrink, before finally calling [`queue_free`][`crate::classes::Node::queue_free`] to free the sprite. [`Tweener`][crate::classes::Tweener]s are executed one after another by default. This behavior can be changed using [`parallel`][`crate::classes::Tween::parallel`] and [`set_parallel`][`crate::classes::Tween::set_parallel`].\n\nWhen a [`Tweener`][crate::classes::Tweener] is created with one of the `tween_*` methods, a chained method call can be used to tweak the properties of this [`Tweener`][crate::classes::Tweener]. For example, if you want to set a different transition type in the above example, you can use [`set_trans`][`crate::classes::Tween::set_trans`]:\n\n\n```gdscript\nvar tween = get_tree().create_tween()\ntween.tween_property($Sprite, \"modulate\", Color.RED, 1.0).set_trans(Tween.TRANS_SINE)\ntween.tween_property($Sprite, \"scale\", Vector2(), 1.0).set_trans(Tween.TRANS_BOUNCE)\ntween.tween_callback($Sprite.queue_free)\n```\n\n\nMost of the `Tween` methods can be chained this way too. In the following example the `Tween` is bound to the running script's node and a default transition is set for its [`Tweener`][crate::classes::Tweener]s:\n\n\n```gdscript\nvar tween = get_tree().create_tween().bind_node(self).set_trans(Tween.TRANS_ELASTIC)\ntween.tween_property($Sprite, \"modulate\", Color.RED, 1.0)\ntween.tween_property($Sprite, \"scale\", Vector2(), 1.0)\ntween.tween_callback($Sprite.queue_free)\n```\n\n\nAnother interesting use for `Tween`s is animating arbitrary sets of objects:\n\n\n```gdscript\nvar tween = create_tween()\nfor sprite in get_children():\n\ttween.tween_property(sprite, \"position\", Vector2(0, 0), 1.0)\n```\n\n\nIn the example above, all children of a node are moved one after another to position `(0, 0)`.\n\nYou should avoid using more than one `Tween` per object's property. If two or more tweens animate one property at the same time, the last one created will take priority and assign the final value. If you want to interrupt and restart an animation, consider assigning the `Tween` to a variable:\n\n\n```gdscript\nvar tween\nfunc animate():\n\tif tween:\n\t\ttween.kill() # Abort the previous animation.\n\ttween = create_tween()\n```\n\n\nSome [`Tweener`][crate::classes::Tweener]s use transitions and eases. The first accepts a \\[enum TransitionType] constant, and refers to the way the timing of the animation is handled (see [easings.net](https://easings.net/) for some examples). The second accepts an \\[enum EaseType] constant, and controls where the `trans_type` is applied to the interpolation (in the beginning, the end, or both). If you don't know which transition and easing to pick, you can try different \\[enum TransitionType] constants with [`EaseType::IN_OUT`][`crate::classes::tween::EaseType::IN_OUT`], and use the one that looks best.\n\n[Tween easing and transition types cheatsheet](https://raw.githubusercontent.com/godotengine/godot-docs/master/img/tween_cheatsheet.webp)\n\n**Note:** Tweens are not designed to be reused and trying to do so results in an undefined behavior. Create a new Tween for each animation and every time you replay an animation from start. Keep in mind that Tweens start immediately, so only create a Tween when you want to start animating.\n\n**Note:** The tween is processed after all of the nodes in the current frame, i.e. node's [`process`][`crate::classes::INode::process`] method would be called before the tween (or [`physics_process`][`crate::classes::INode::physics_process`] depending on the value passed to [`set_process_mode`][`crate::classes::Tween::set_process_mode`])."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Tween {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`Tween`][crate::classes::Tween].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `Tween` methods](https://docs.godotengine.org/en/stable/classes/class_tween.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ITween: crate::obj::GodotClass < Base = Tween > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl Tween {
        #[doc = "Creates and appends a [`PropertyTweener`][crate::classes::PropertyTweener]. This method tweens a `property` of an `object` between an initial value and `final_val` in a span of time equal to `duration`, in seconds. The initial value by default is the property's value at the time the tweening of the [`PropertyTweener`][crate::classes::PropertyTweener] starts.\n\n\n```gdscript\nvar tween = create_tween()\ntween.tween_property($Sprite, \"position\", Vector2(100, 200), 1.0)\ntween.tween_property($Sprite, \"position\", Vector2(200, 300), 1.0)\n```\n\n\nwill move the sprite to position (100, 200) and then to (200, 300). If you use [`from`][`crate::classes::PropertyTweener::from`] or [`from_current`][`crate::classes::PropertyTweener::from_current`], the starting position will be overwritten by the given value instead. See other methods in [`PropertyTweener`][crate::classes::PropertyTweener] to see how the tweening can be tweaked further.\n\n**Note:** You can find the correct property name by hovering over the property in the Inspector. You can also provide the components of a property directly by using `\"property:component\"` (eg. `position:x`), where it would only apply to that particular component.\n\n**Example:** Moving an object twice from the same position, with different transition types:\n\n\n```gdscript\nvar tween = create_tween()\ntween.tween_property($Sprite, \"position\", Vector2.RIGHT * 300, 1.0).as_relative().set_trans(Tween.TRANS_SINE)\ntween.tween_property($Sprite, \"position\", Vector2.RIGHT * 300, 1.0).as_relative().from_current().set_trans(Tween.TRANS_EXPO)\n```\n"]
        pub fn tween_property(&mut self, object: impl AsArg < Gd < crate::classes::Object >>, property: impl AsArg < NodePath >, final_val: &Variant, duration: f64,) -> Gd < crate::classes::PropertyTweener > {
            type CallRet = Gd < crate::classes::PropertyTweener >;
            type CallParams < 'a0, 'a1, 'a2, > = (CowArg < 'a0, Gd < crate::classes::Object > >, CowArg < 'a1, NodePath >, RefArg < 'a2, Variant >, f64,);
            let args = (object.into_arg(), property.into_arg(), RefArg::new(final_val), duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(60usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "tween_property", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates and appends an [`IntervalTweener`][crate::classes::IntervalTweener]. This method can be used to create delays in the tween animation, as an alternative to using the delay in other [`Tweener`][crate::classes::Tweener]s, or when there's no animation (in which case the `Tween` acts as a timer). `time` is the length of the interval, in seconds.\n\n**Example:** Creating an interval in code execution:\n\n\n```gdscript\n# ... some code\nawait create_tween().tween_interval(2).finished\n# ... more code\n```\n\n\n**Example:** Creating an object that moves back and forth and jumps every few seconds:\n\n\n```gdscript\nvar tween = create_tween().set_loops()\ntween.tween_property($Sprite, \"position:x\", 200.0, 1.0).as_relative()\ntween.tween_callback(jump)\ntween.tween_interval(2)\ntween.tween_property($Sprite, \"position:x\", -200.0, 1.0).as_relative()\ntween.tween_callback(jump)\ntween.tween_interval(2)\n```\n"]
        pub fn tween_interval(&mut self, time: f64,) -> Gd < crate::classes::IntervalTweener > {
            type CallRet = Gd < crate::classes::IntervalTweener >;
            type CallParams = (f64,);
            let args = (time,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(61usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "tween_interval", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates and appends a [`CallbackTweener`][crate::classes::CallbackTweener]. This method can be used to call an arbitrary method in any object. Use [`bind`][`crate::builtin::Callable::bind`] to bind additional arguments for the call.\n\n**Example:** Object that keeps shooting every 1 second:\n\n\n```gdscript\nvar tween = get_tree().create_tween().set_loops()\ntween.tween_callback(shoot).set_delay(1.0)\n```\n\n\n**Example:** Turning a sprite red and then blue, with 2 second delay:\n\n\n```gdscript\nvar tween = get_tree().create_tween()\ntween.tween_callback($Sprite.set_modulate.bind(Color.RED)).set_delay(2)\ntween.tween_callback($Sprite.set_modulate.bind(Color.BLUE)).set_delay(2)\n```\n"]
        pub fn tween_callback(&mut self, callback: &Callable,) -> Gd < crate::classes::CallbackTweener > {
            type CallRet = Gd < crate::classes::CallbackTweener >;
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(62usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "tween_callback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates and appends a [`MethodTweener`][crate::classes::MethodTweener]. This method is similar to a combination of [`tween_callback`][`crate::classes::Tween::tween_callback`] and [`tween_property`][`crate::classes::Tween::tween_property`]. It calls a method over time with a tweened value provided as an argument. The value is tweened between `from` and `to` over the time specified by `duration`, in seconds. Use [`bind`][`crate::builtin::Callable::bind`] to bind additional arguments for the call. You can use [`set_ease`][`crate::classes::MethodTweener::set_ease`] and [`set_trans`][`crate::classes::MethodTweener::set_trans`] to tweak the easing and transition of the value or [`set_delay`][`crate::classes::MethodTweener::set_delay`] to delay the tweening.\n\n**Example:** Making a 3D object look from one point to another point:\n\n\n```gdscript\nvar tween = create_tween()\ntween.tween_method(look_at.bind(Vector3.UP), Vector3(-1, 0, -1), Vector3(1, 0, -1), 1.0) # The look_at() method takes up vector as second argument.\n```\n\n\n**Example:** Setting the text of a [`Label`][crate::classes::Label], using an intermediate method and after a delay:\n\n\n```gdscript\nfunc _ready():\n\tvar tween = create_tween()\n\ttween.tween_method(set_label_text, 0, 10, 1.0).set_delay(1.0)\n\nfunc set_label_text(value: int):\n\t$Label.text = \"Counting \" + str(value)\n```\n"]
        pub fn tween_method(&mut self, method: &Callable, from: &Variant, to: &Variant, duration: f64,) -> Gd < crate::classes::MethodTweener > {
            type CallRet = Gd < crate::classes::MethodTweener >;
            type CallParams < 'a0, 'a1, 'a2, > = (RefArg < 'a0, Callable >, RefArg < 'a1, Variant >, RefArg < 'a2, Variant >, f64,);
            let args = (RefArg::new(method), RefArg::new(from), RefArg::new(to), duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(63usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "tween_method", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Creates and appends a [`SubtweenTweener`][crate::classes::SubtweenTweener]. This method can be used to nest `subtween` within this `Tween`, allowing for the creation of more complex and composable sequences.\n\n```gdscript\n# Subtween will rotate the object.\nvar subtween = create_tween()\nsubtween.tween_property(self, \"rotation_degrees\", 45.0, 1.0)\nsubtween.tween_property(self, \"rotation_degrees\", 0.0, 1.0)\n\n# Parent tween will execute the subtween as one of its steps.\nvar tween = create_tween()\ntween.tween_property(self, \"position:x\", 500, 3.0)\ntween.tween_subtween(subtween)\ntween.tween_property(self, \"position:x\", 300, 2.0)\n```\n\n**Note:** The methods [`pause`][`crate::classes::Tween::pause`], [`stop`][`crate::classes::Tween::stop`], and [`set_loops`][`crate::classes::Tween::set_loops`] can cause the parent `Tween` to get stuck on the subtween step; see the documentation for those methods for more information.\n\n**Note:** The pause and process modes set by [`set_pause_mode`][`crate::classes::Tween::set_pause_mode`] and [`set_process_mode`][`crate::classes::Tween::set_process_mode`] on `subtween` will be overridden by the parent `Tween`'s settings."]
        pub fn tween_subtween(&mut self, subtween: impl AsArg < Gd < crate::classes::Tween >>,) -> Gd < crate::classes::SubtweenTweener > {
            type CallRet = Gd < crate::classes::SubtweenTweener >;
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::Tween > >,);
            let args = (subtween.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(64usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "tween_subtween", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Processes the `Tween` by the given `delta` value, in seconds. This is mostly useful for manual control when the `Tween` is paused. It can also be used to end the `Tween` animation immediately, by setting `delta` longer than the whole duration of the `Tween` animation.\n\nReturns `true` if the `Tween` still has [`Tweener`][crate::classes::Tweener]s that haven't finished."]
        pub fn custom_step(&mut self, delta: f64,) -> bool {
            type CallRet = bool;
            type CallParams = (f64,);
            let args = (delta,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(65usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "custom_step", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Stops the tweening and resets the `Tween` to its initial state. This will not remove any appended [`Tweener`][crate::classes::Tweener]s.\n\n**Note:** This does _not_ reset targets of [`PropertyTweener`][crate::classes::PropertyTweener]s to their values when the `Tween` first started.\n\n```gdscript\nvar tween = create_tween()\n\n# Will move from 0 to 500 over 1 second.\nposition.x = 0.0\ntween.tween_property(self, \"position:x\", 500, 1.0)\n\n# Will be at (about) 250 when the timer finishes.\nawait get_tree().create_timer(0.5).timeout\n\n# Will now move from (about) 250 to 500 over 1 second,\n# thus at half the speed as before.\ntween.stop()\ntween.play()\n```\n\n**Note:** If a Tween is stopped and not bound to any node, it will exist indefinitely until manually started or invalidated. If you lose a reference to such Tween, you can retrieve it using [`get_processed_tweens`][`crate::classes::SceneTree::get_processed_tweens`]."]
        pub fn stop(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(66usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "stop", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Pauses the tweening. The animation can be resumed by using [`play`][`crate::classes::Tween::play`].\n\n**Note:** If a Tween is paused and not bound to any node, it will exist indefinitely until manually started or invalidated. If you lose a reference to such Tween, you can retrieve it using [`get_processed_tweens`][`crate::classes::SceneTree::get_processed_tweens`]."]
        pub fn pause(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(67usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "pause", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Resumes a paused or stopped `Tween`."]
        pub fn play(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(68usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "play", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Aborts all tweening operations and invalidates the `Tween`."]
        pub fn kill(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(69usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "kill", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the total time in seconds the `Tween` has been animating (i.e. the time since it started, not counting pauses etc.). The time is affected by [`set_speed_scale`][`crate::classes::Tween::set_speed_scale`], and [`stop`][`crate::classes::Tween::stop`] will reset it to `0`.\n\n**Note:** As it results from accumulating frame deltas, the time returned after the `Tween` has finished animating will be slightly greater than the actual `Tween` duration."]
        pub fn get_total_elapsed_time(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(70usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "get_total_elapsed_time", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns whether the `Tween` is currently running, i.e. it wasn't paused and it's not finished."]
        pub fn is_running(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(71usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "is_running", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns whether the `Tween` is valid. A valid `Tween` is a `Tween` contained by the scene tree (i.e. the array from [`get_processed_tweens`][`crate::classes::SceneTree::get_processed_tweens`] will contain this `Tween`). A `Tween` might become invalid when it has finished tweening, is killed, or when created with `Tween.new()`. Invalid `Tween`s can't have [`Tweener`][crate::classes::Tweener]s appended."]
        pub fn is_valid(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(72usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "is_valid", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Binds this `Tween` with the given `node`. `Tween`s are processed directly by the [`SceneTree`][crate::classes::SceneTree], so they run independently of the animated nodes. When you bind a [`Node`][crate::classes::Node] with the `Tween`, the `Tween` will halt the animation when the object is not inside tree and the `Tween` will be automatically killed when the bound object is freed. Also [`TweenPauseMode::BOUND`][`crate::classes::tween::TweenPauseMode::BOUND`] will make the pausing behavior dependent on the bound node.\n\nFor a shorter way to create and bind a `Tween`, you can use [`create_tween`][`crate::classes::Node::create_tween`]."]
        pub fn bind_node(&mut self, node: impl AsArg < Gd < crate::classes::Node >>,) -> Gd < crate::classes::Tween > {
            type CallRet = Gd < crate::classes::Tween >;
            type CallParams < 'a0, > = (CowArg < 'a0, Gd < crate::classes::Node > >,);
            let args = (node.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(73usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "bind_node", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Determines whether the `Tween` should run after process frames (see [`process`][`crate::classes::INode::process`]) or physics frames (see [`physics_process`][`crate::classes::INode::physics_process`]).\n\nDefault value is [`TweenProcessMode::IDLE`][`crate::classes::tween::TweenProcessMode::IDLE`]."]
        pub fn set_process_mode(&mut self, mode: crate::classes::tween::TweenProcessMode,) -> Gd < crate::classes::Tween > {
            type CallRet = Gd < crate::classes::Tween >;
            type CallParams = (crate::classes::tween::TweenProcessMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(74usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "set_process_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Determines the behavior of the `Tween` when the [`SceneTree`][crate::classes::SceneTree] is paused.\n\nDefault value is [`TweenPauseMode::BOUND`][`crate::classes::tween::TweenPauseMode::BOUND`]."]
        pub fn set_pause_mode(&mut self, mode: crate::classes::tween::TweenPauseMode,) -> Gd < crate::classes::Tween > {
            type CallRet = Gd < crate::classes::Tween >;
            type CallParams = (crate::classes::tween::TweenPauseMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(75usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "set_pause_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `ignore` is `true`, the tween will ignore \\[member Engine.time_scale] and update with the real, elapsed time. This affects all [`Tweener`][crate::classes::Tweener]s and their delays. Default value is `false`."]
        pub(crate) fn set_ignore_time_scale_full(&mut self, ignore: bool,) -> Gd < crate::classes::Tween > {
            type CallRet = Gd < crate::classes::Tween >;
            type CallParams = (bool,);
            let args = (ignore,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(76usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "set_ignore_time_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_ignore_time_scale_ex`][Self::set_ignore_time_scale_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "If `ignore` is `true`, the tween will ignore \\[member Engine.time_scale] and update with the real, elapsed time. This affects all [`Tweener`][crate::classes::Tweener]s and their delays. Default value is `false`."]
        #[inline]
        pub fn set_ignore_time_scale(&mut self,) -> Gd < crate::classes::Tween > {
            self.set_ignore_time_scale_ex() . done()
        }
        #[doc = "If `ignore` is `true`, the tween will ignore \\[member Engine.time_scale] and update with the real, elapsed time. This affects all [`Tweener`][crate::classes::Tweener]s and their delays. Default value is `false`."]
        #[inline]
        pub fn set_ignore_time_scale_ex < 'ex > (&'ex mut self,) -> ExSetIgnoreTimeScale < 'ex > {
            ExSetIgnoreTimeScale::new(self,)
        }
        #[doc = "If `parallel` is `true`, the [`Tweener`][crate::classes::Tweener]s appended after this method will by default run simultaneously, as opposed to sequentially.\n\n**Note:** Just like with [`parallel`][`crate::classes::Tween::parallel`], the tweener added right before this method will also be part of the parallel step.\n\n```gdscript\ntween.tween_property(self, \"position\", Vector2(300, 0), 0.5)\ntween.set_parallel()\ntween.tween_property(self, \"modulate\", Color.GREEN, 0.5) # Runs together with the position tweener.\n```"]
        pub(crate) fn set_parallel_full(&mut self, parallel: bool,) -> Gd < crate::classes::Tween > {
            type CallRet = Gd < crate::classes::Tween >;
            type CallParams = (bool,);
            let args = (parallel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(77usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "set_parallel", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_parallel_ex`][Self::set_parallel_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "If `parallel` is `true`, the [`Tweener`][crate::classes::Tweener]s appended after this method will by default run simultaneously, as opposed to sequentially.\n\n**Note:** Just like with [`parallel`][`crate::classes::Tween::parallel`], the tweener added right before this method will also be part of the parallel step.\n\n```gdscript\ntween.tween_property(self, \"position\", Vector2(300, 0), 0.5)\ntween.set_parallel()\ntween.tween_property(self, \"modulate\", Color.GREEN, 0.5) # Runs together with the position tweener.\n```"]
        #[inline]
        pub fn set_parallel(&mut self,) -> Gd < crate::classes::Tween > {
            self.set_parallel_ex() . done()
        }
        #[doc = "If `parallel` is `true`, the [`Tweener`][crate::classes::Tweener]s appended after this method will by default run simultaneously, as opposed to sequentially.\n\n**Note:** Just like with [`parallel`][`crate::classes::Tween::parallel`], the tweener added right before this method will also be part of the parallel step.\n\n```gdscript\ntween.tween_property(self, \"position\", Vector2(300, 0), 0.5)\ntween.set_parallel()\ntween.tween_property(self, \"modulate\", Color.GREEN, 0.5) # Runs together with the position tweener.\n```"]
        #[inline]
        pub fn set_parallel_ex < 'ex > (&'ex mut self,) -> ExSetParallel < 'ex > {
            ExSetParallel::new(self,)
        }
        #[doc = "Sets the number of times the tweening sequence will be repeated, i.e. `set_loops(2)` will run the animation twice.\n\nCalling this method without arguments will make the `Tween` run infinitely, until either it is killed with [`kill`][`crate::classes::Tween::kill`], the `Tween`'s bound node is freed, or all the animated objects have been freed (which makes further animation impossible).\n\n**Warning:** Make sure to always add some duration/delay when using infinite loops. To prevent the game freezing, 0-duration looped animations (e.g. a single [`CallbackTweener`][crate::classes::CallbackTweener] with no delay) are stopped after a small number of loops, which may produce unexpected results. If a `Tween`'s lifetime depends on some node, always use [`bind_node`][`crate::classes::Tween::bind_node`]."]
        pub(crate) fn set_loops_full(&mut self, loops: i32,) -> Gd < crate::classes::Tween > {
            type CallRet = Gd < crate::classes::Tween >;
            type CallParams = (i32,);
            let args = (loops,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(78usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "set_loops", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_loops_ex`][Self::set_loops_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the number of times the tweening sequence will be repeated, i.e. `set_loops(2)` will run the animation twice.\n\nCalling this method without arguments will make the `Tween` run infinitely, until either it is killed with [`kill`][`crate::classes::Tween::kill`], the `Tween`'s bound node is freed, or all the animated objects have been freed (which makes further animation impossible).\n\n**Warning:** Make sure to always add some duration/delay when using infinite loops. To prevent the game freezing, 0-duration looped animations (e.g. a single [`CallbackTweener`][crate::classes::CallbackTweener] with no delay) are stopped after a small number of loops, which may produce unexpected results. If a `Tween`'s lifetime depends on some node, always use [`bind_node`][`crate::classes::Tween::bind_node`]."]
        #[inline]
        pub fn set_loops(&mut self,) -> Gd < crate::classes::Tween > {
            self.set_loops_ex() . done()
        }
        #[doc = "Sets the number of times the tweening sequence will be repeated, i.e. `set_loops(2)` will run the animation twice.\n\nCalling this method without arguments will make the `Tween` run infinitely, until either it is killed with [`kill`][`crate::classes::Tween::kill`], the `Tween`'s bound node is freed, or all the animated objects have been freed (which makes further animation impossible).\n\n**Warning:** Make sure to always add some duration/delay when using infinite loops. To prevent the game freezing, 0-duration looped animations (e.g. a single [`CallbackTweener`][crate::classes::CallbackTweener] with no delay) are stopped after a small number of loops, which may produce unexpected results. If a `Tween`'s lifetime depends on some node, always use [`bind_node`][`crate::classes::Tween::bind_node`]."]
        #[inline]
        pub fn set_loops_ex < 'ex > (&'ex mut self,) -> ExSetLoops < 'ex > {
            ExSetLoops::new(self,)
        }
        #[doc = "Returns the number of remaining loops for this `Tween` (see [`set_loops`][`crate::classes::Tween::set_loops`]). A return value of `-1` indicates an infinitely looping `Tween`, and a return value of `0` indicates that the `Tween` has already finished."]
        pub fn get_loops_left(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(79usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "get_loops_left", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Scales the speed of tweening. This affects all [`Tweener`][crate::classes::Tweener]s and their delays."]
        pub fn set_speed_scale(&mut self, speed: f32,) -> Gd < crate::classes::Tween > {
            type CallRet = Gd < crate::classes::Tween >;
            type CallParams = (f32,);
            let args = (speed,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(80usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "set_speed_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the default transition type for [`PropertyTweener`][crate::classes::PropertyTweener]s and [`MethodTweener`][crate::classes::MethodTweener]s appended after this method.\n\nBefore this method is called, the default transition type is [`TransitionType::LINEAR`][`crate::classes::tween::TransitionType::LINEAR`].\n\n```gdscript\nvar tween = create_tween()\ntween.tween_property(self, \"position\", Vector2(300, 0), 0.5) # Uses TRANS_LINEAR.\ntween.set_trans(Tween.TRANS_SINE)\ntween.tween_property(self, \"rotation_degrees\", 45.0, 0.5) # Uses TRANS_SINE.\n```"]
        pub fn set_trans(&mut self, trans: crate::classes::tween::TransitionType,) -> Gd < crate::classes::Tween > {
            type CallRet = Gd < crate::classes::Tween >;
            type CallParams = (crate::classes::tween::TransitionType,);
            let args = (trans,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(81usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "set_trans", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the default ease type for [`PropertyTweener`][crate::classes::PropertyTweener]s and [`MethodTweener`][crate::classes::MethodTweener]s appended after this method.\n\nBefore this method is called, the default ease type is [`EaseType::IN_OUT`][`crate::classes::tween::EaseType::IN_OUT`].\n\n```gdscript\nvar tween = create_tween()\ntween.tween_property(self, \"position\", Vector2(300, 0), 0.5) # Uses EASE_IN_OUT.\ntween.set_ease(Tween.EASE_IN)\ntween.tween_property(self, \"rotation_degrees\", 45.0, 0.5) # Uses EASE_IN.\n```"]
        pub fn set_ease(&mut self, ease: crate::classes::tween::EaseType,) -> Gd < crate::classes::Tween > {
            type CallRet = Gd < crate::classes::Tween >;
            type CallParams = (crate::classes::tween::EaseType,);
            let args = (ease,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(82usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "set_ease", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Makes the next [`Tweener`][crate::classes::Tweener] run parallelly to the previous one.\n\n\n```gdscript\nvar tween = create_tween()\ntween.tween_property(...)\ntween.parallel().tween_property(...)\ntween.parallel().tween_property(...)\n```\n\n\nAll [`Tweener`][crate::classes::Tweener]s in the example will run at the same time.\n\nYou can make the `Tween` parallel by default by using [`set_parallel`][`crate::classes::Tween::set_parallel`]."]
        pub fn parallel(&mut self,) -> Gd < crate::classes::Tween > {
            type CallRet = Gd < crate::classes::Tween >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(83usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "parallel", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Used to chain two [`Tweener`][crate::classes::Tweener]s after [`set_parallel`][`crate::classes::Tween::set_parallel`] is called with `true`.\n\n\n```gdscript\nvar tween = create_tween().set_parallel(true)\ntween.tween_property(...)\ntween.tween_property(...) # Will run parallelly with above.\ntween.chain().tween_property(...) # Will run after two above are finished.\n```\n"]
        pub fn chain(&mut self,) -> Gd < crate::classes::Tween > {
            type CallRet = Gd < crate::classes::Tween >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(84usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "chain", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "This method can be used for manual interpolation of a value, when you don't want `Tween` to do animating for you. It's similar to [`lerp`][`crate::global::lerp`], but with support for custom transition and easing.\n\n`initial_value` is the starting value of the interpolation.\n\n`delta_value` is the change of the value in the interpolation, i.e. it's equal to `final_value - initial_value`.\n\n`elapsed_time` is the time in seconds that passed after the interpolation started and it's used to control the position of the interpolation. E.g. when it's equal to half of the `duration`, the interpolated value will be halfway between initial and final values. This value can also be greater than `duration` or lower than 0, which will extrapolate the value.\n\n`duration` is the total time of the interpolation.\n\n**Note:** If `duration` is equal to `0`, the method will always return the final value, regardless of `elapsed_time` provided."]
        pub fn interpolate_value(initial_value: &Variant, delta_value: &Variant, elapsed_time: f64, duration: f64, trans_type: crate::classes::tween::TransitionType, ease_type: crate::classes::tween::EaseType,) -> Variant {
            type CallRet = Variant;
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, Variant >, RefArg < 'a1, Variant >, f64, f64, crate::classes::tween::TransitionType, crate::classes::tween::EaseType,);
            let args = (RefArg::new(initial_value), RefArg::new(delta_value), elapsed_time, duration, trans_type, ease_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(85usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Tween", "interpolate_value", None, args,)
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
    impl crate::obj::GodotClass for Tween {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("Tween"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Tween {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for Tween {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Tween {
        
    }
    impl std::ops::Deref for Tween {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Tween {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`Tween`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Tween__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::Tween > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`Tween::set_ignore_time_scale_ex`][super::Tween::set_ignore_time_scale_ex]."]
#[must_use]
pub struct ExSetIgnoreTimeScale < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Tween, ignore: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetIgnoreTimeScale < 'ex > {
    fn new(surround_object: &'ex mut re_export::Tween,) -> Self {
        let ignore = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, ignore: ignore,
        }
    }
    #[inline]
    pub fn ignore(self, ignore: bool) -> Self {
        Self {
            ignore: ignore, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Gd < crate::classes::Tween > {
        let Self {
            _phantom, surround_object, ignore,
        }
        = self;
        re_export::Tween::set_ignore_time_scale_full(surround_object, ignore,)
    }
}
#[doc = "Default-param extender for [`Tween::set_parallel_ex`][super::Tween::set_parallel_ex]."]
#[must_use]
pub struct ExSetParallel < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Tween, parallel: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetParallel < 'ex > {
    fn new(surround_object: &'ex mut re_export::Tween,) -> Self {
        let parallel = true;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, parallel: parallel,
        }
    }
    #[inline]
    pub fn parallel(self, parallel: bool) -> Self {
        Self {
            parallel: parallel, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Gd < crate::classes::Tween > {
        let Self {
            _phantom, surround_object, parallel,
        }
        = self;
        re_export::Tween::set_parallel_full(surround_object, parallel,)
    }
}
#[doc = "Default-param extender for [`Tween::set_loops_ex`][super::Tween::set_loops_ex]."]
#[must_use]
pub struct ExSetLoops < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Tween, loops: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetLoops < 'ex > {
    fn new(surround_object: &'ex mut re_export::Tween,) -> Self {
        let loops = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, loops: loops,
        }
    }
    #[inline]
    pub fn loops(self, loops: i32) -> Self {
        Self {
            loops: loops, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Gd < crate::classes::Tween > {
        let Self {
            _phantom, surround_object, loops,
        }
        = self;
        re_export::Tween::set_loops_full(surround_object, loops,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TweenProcessMode {
    ord: i32
}
impl TweenProcessMode {
    #[doc(alias = "TWEEN_PROCESS_PHYSICS")]
    #[doc = "Godot enumerator name: `TWEEN_PROCESS_PHYSICS`"]
    pub const PHYSICS: TweenProcessMode = TweenProcessMode {
        ord: 0i32
    };
    #[doc(alias = "TWEEN_PROCESS_IDLE")]
    #[doc = "Godot enumerator name: `TWEEN_PROCESS_IDLE`"]
    pub const IDLE: TweenProcessMode = TweenProcessMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for TweenProcessMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TweenProcessMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TweenProcessMode {
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
            Self::PHYSICS => "PHYSICS", Self::IDLE => "IDLE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TweenProcessMode::PHYSICS, TweenProcessMode::IDLE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TweenProcessMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("PHYSICS", "TWEEN_PROCESS_PHYSICS", TweenProcessMode::PHYSICS), crate::meta::inspect::EnumConstant::new("IDLE", "TWEEN_PROCESS_IDLE", TweenProcessMode::IDLE)]
        }
    }
}
impl crate::meta::GodotConvert for TweenProcessMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Tween Process Physics", 0i64), EnumeratorShape::new_int("Tween Process Idle", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Tween.TweenProcessMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for TweenProcessMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TweenProcessMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for TweenProcessMode {
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
impl crate::registry::property::Export for TweenProcessMode {
    
}
impl crate::meta::Element for TweenProcessMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TweenPauseMode {
    ord: i32
}
impl TweenPauseMode {
    #[doc(alias = "TWEEN_PAUSE_BOUND")]
    #[doc = "Godot enumerator name: `TWEEN_PAUSE_BOUND`"]
    pub const BOUND: TweenPauseMode = TweenPauseMode {
        ord: 0i32
    };
    #[doc(alias = "TWEEN_PAUSE_STOP")]
    #[doc = "Godot enumerator name: `TWEEN_PAUSE_STOP`"]
    pub const STOP: TweenPauseMode = TweenPauseMode {
        ord: 1i32
    };
    #[doc(alias = "TWEEN_PAUSE_PROCESS")]
    #[doc = "Godot enumerator name: `TWEEN_PAUSE_PROCESS`"]
    pub const PROCESS: TweenPauseMode = TweenPauseMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TweenPauseMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TweenPauseMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TweenPauseMode {
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
            Self::BOUND => "BOUND", Self::STOP => "STOP", Self::PROCESS => "PROCESS", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TweenPauseMode::BOUND, TweenPauseMode::STOP, TweenPauseMode::PROCESS]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TweenPauseMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("BOUND", "TWEEN_PAUSE_BOUND", TweenPauseMode::BOUND), crate::meta::inspect::EnumConstant::new("STOP", "TWEEN_PAUSE_STOP", TweenPauseMode::STOP), crate::meta::inspect::EnumConstant::new("PROCESS", "TWEEN_PAUSE_PROCESS", TweenPauseMode::PROCESS)]
        }
    }
}
impl crate::meta::GodotConvert for TweenPauseMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Tween Pause Bound", 0i64), EnumeratorShape::new_int("Tween Pause Stop", 1i64), EnumeratorShape::new_int("Tween Pause Process", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Tween.TweenPauseMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for TweenPauseMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TweenPauseMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for TweenPauseMode {
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
impl crate::registry::property::Export for TweenPauseMode {
    
}
impl crate::meta::Element for TweenPauseMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TransitionType {
    ord: i32
}
impl TransitionType {
    #[doc(alias = "TRANS_LINEAR")]
    #[doc = "Godot enumerator name: `TRANS_LINEAR`"]
    pub const LINEAR: TransitionType = TransitionType {
        ord: 0i32
    };
    #[doc(alias = "TRANS_SINE")]
    #[doc = "Godot enumerator name: `TRANS_SINE`"]
    pub const SINE: TransitionType = TransitionType {
        ord: 1i32
    };
    #[doc(alias = "TRANS_QUINT")]
    #[doc = "Godot enumerator name: `TRANS_QUINT`"]
    pub const QUINT: TransitionType = TransitionType {
        ord: 2i32
    };
    #[doc(alias = "TRANS_QUART")]
    #[doc = "Godot enumerator name: `TRANS_QUART`"]
    pub const QUART: TransitionType = TransitionType {
        ord: 3i32
    };
    #[doc(alias = "TRANS_QUAD")]
    #[doc = "Godot enumerator name: `TRANS_QUAD`"]
    pub const QUAD: TransitionType = TransitionType {
        ord: 4i32
    };
    #[doc(alias = "TRANS_EXPO")]
    #[doc = "Godot enumerator name: `TRANS_EXPO`"]
    pub const EXPO: TransitionType = TransitionType {
        ord: 5i32
    };
    #[doc(alias = "TRANS_ELASTIC")]
    #[doc = "Godot enumerator name: `TRANS_ELASTIC`"]
    pub const ELASTIC: TransitionType = TransitionType {
        ord: 6i32
    };
    #[doc(alias = "TRANS_CUBIC")]
    #[doc = "Godot enumerator name: `TRANS_CUBIC`"]
    pub const CUBIC: TransitionType = TransitionType {
        ord: 7i32
    };
    #[doc(alias = "TRANS_CIRC")]
    #[doc = "Godot enumerator name: `TRANS_CIRC`"]
    pub const CIRC: TransitionType = TransitionType {
        ord: 8i32
    };
    #[doc(alias = "TRANS_BOUNCE")]
    #[doc = "Godot enumerator name: `TRANS_BOUNCE`"]
    pub const BOUNCE: TransitionType = TransitionType {
        ord: 9i32
    };
    #[doc(alias = "TRANS_BACK")]
    #[doc = "Godot enumerator name: `TRANS_BACK`"]
    pub const BACK: TransitionType = TransitionType {
        ord: 10i32
    };
    #[doc(alias = "TRANS_SPRING")]
    #[doc = "Godot enumerator name: `TRANS_SPRING`"]
    pub const SPRING: TransitionType = TransitionType {
        ord: 11i32
    };
    
}
impl std::fmt::Debug for TransitionType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TransitionType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TransitionType {
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
            Self::LINEAR => "LINEAR", Self::SINE => "SINE", Self::QUINT => "QUINT", Self::QUART => "QUART", Self::QUAD => "QUAD", Self::EXPO => "EXPO", Self::ELASTIC => "ELASTIC", Self::CUBIC => "CUBIC", Self::CIRC => "CIRC", Self::BOUNCE => "BOUNCE", Self::BACK => "BACK", Self::SPRING => "SPRING", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TransitionType::LINEAR, TransitionType::SINE, TransitionType::QUINT, TransitionType::QUART, TransitionType::QUAD, TransitionType::EXPO, TransitionType::ELASTIC, TransitionType::CUBIC, TransitionType::CIRC, TransitionType::BOUNCE, TransitionType::BACK, TransitionType::SPRING]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TransitionType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("LINEAR", "TRANS_LINEAR", TransitionType::LINEAR), crate::meta::inspect::EnumConstant::new("SINE", "TRANS_SINE", TransitionType::SINE), crate::meta::inspect::EnumConstant::new("QUINT", "TRANS_QUINT", TransitionType::QUINT), crate::meta::inspect::EnumConstant::new("QUART", "TRANS_QUART", TransitionType::QUART), crate::meta::inspect::EnumConstant::new("QUAD", "TRANS_QUAD", TransitionType::QUAD), crate::meta::inspect::EnumConstant::new("EXPO", "TRANS_EXPO", TransitionType::EXPO), crate::meta::inspect::EnumConstant::new("ELASTIC", "TRANS_ELASTIC", TransitionType::ELASTIC), crate::meta::inspect::EnumConstant::new("CUBIC", "TRANS_CUBIC", TransitionType::CUBIC), crate::meta::inspect::EnumConstant::new("CIRC", "TRANS_CIRC", TransitionType::CIRC), crate::meta::inspect::EnumConstant::new("BOUNCE", "TRANS_BOUNCE", TransitionType::BOUNCE), crate::meta::inspect::EnumConstant::new("BACK", "TRANS_BACK", TransitionType::BACK), crate::meta::inspect::EnumConstant::new("SPRING", "TRANS_SPRING", TransitionType::SPRING)]
        }
    }
}
impl crate::meta::GodotConvert for TransitionType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Trans Linear", 0i64), EnumeratorShape::new_int("Trans Sine", 1i64), EnumeratorShape::new_int("Trans Quint", 2i64), EnumeratorShape::new_int("Trans Quart", 3i64), EnumeratorShape::new_int("Trans Quad", 4i64), EnumeratorShape::new_int("Trans Expo", 5i64), EnumeratorShape::new_int("Trans Elastic", 6i64), EnumeratorShape::new_int("Trans Cubic", 7i64), EnumeratorShape::new_int("Trans Circ", 8i64), EnumeratorShape::new_int("Trans Bounce", 9i64), EnumeratorShape::new_int("Trans Back", 10i64), EnumeratorShape::new_int("Trans Spring", 11i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Tween.TransitionType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for TransitionType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TransitionType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for TransitionType {
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
impl crate::registry::property::Export for TransitionType {
    
}
impl crate::meta::Element for TransitionType {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct EaseType {
    ord: i32
}
impl EaseType {
    #[doc(alias = "EASE_IN")]
    #[doc = "Godot enumerator name: `EASE_IN`"]
    pub const IN: EaseType = EaseType {
        ord: 0i32
    };
    #[doc(alias = "EASE_OUT")]
    #[doc = "Godot enumerator name: `EASE_OUT`"]
    pub const OUT: EaseType = EaseType {
        ord: 1i32
    };
    #[doc(alias = "EASE_IN_OUT")]
    #[doc = "Godot enumerator name: `EASE_IN_OUT`"]
    pub const IN_OUT: EaseType = EaseType {
        ord: 2i32
    };
    #[doc(alias = "EASE_OUT_IN")]
    #[doc = "Godot enumerator name: `EASE_OUT_IN`"]
    pub const OUT_IN: EaseType = EaseType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for EaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("EaseType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for EaseType {
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
            Self::IN => "IN", Self::OUT => "OUT", Self::IN_OUT => "IN_OUT", Self::OUT_IN => "OUT_IN", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[EaseType::IN, EaseType::OUT, EaseType::IN_OUT, EaseType::OUT_IN]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < EaseType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("IN", "EASE_IN", EaseType::IN), crate::meta::inspect::EnumConstant::new("OUT", "EASE_OUT", EaseType::OUT), crate::meta::inspect::EnumConstant::new("IN_OUT", "EASE_IN_OUT", EaseType::IN_OUT), crate::meta::inspect::EnumConstant::new("OUT_IN", "EASE_OUT_IN", EaseType::OUT_IN)]
        }
    }
}
impl crate::meta::GodotConvert for EaseType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Ease In", 0i64), EnumeratorShape::new_int("Ease Out", 1i64), EnumeratorShape::new_int("Ease In Out", 2i64), EnumeratorShape::new_int("Ease Out In", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("Tween.EaseType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for EaseType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for EaseType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for EaseType {
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
impl crate::registry::property::Export for EaseType {
    
}
impl crate::meta::Element for EaseType {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Tween;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`Tween`][crate::classes::Tween] class."]
    pub struct SignalsOfTween < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfTween < 'c, C > {
        #[doc = "Signature: `(idx: i64)`"]
        pub fn step_finished(&mut self) -> SigStepFinished < 'c, C > {
            SigStepFinished {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "step_finished")
            }
        }
        #[doc = "Signature: `(loop_count: i64)`"]
        pub fn loop_finished(&mut self) -> SigLoopFinished < 'c, C > {
            SigLoopFinished {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "loop_finished")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn finished(&mut self) -> SigFinished < 'c, C > {
            SigFinished {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "finished")
            }
        }
    }
    type TypedSigStepFinished < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigStepFinished < 'c, C: WithSignals > {
        typed: TypedSigStepFinished < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigStepFinished < 'c, C > {
        pub fn emit(&mut self, idx: i64,) {
            self.typed.emit_tuple((idx,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigStepFinished < 'c, C > {
        type Target = TypedSigStepFinished < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigStepFinished < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigLoopFinished < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigLoopFinished < 'c, C: WithSignals > {
        typed: TypedSigLoopFinished < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigLoopFinished < 'c, C > {
        pub fn emit(&mut self, loop_count: i64,) {
            self.typed.emit_tuple((loop_count,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigLoopFinished < 'c, C > {
        type Target = TypedSigLoopFinished < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigLoopFinished < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigFinished < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigFinished < 'c, C: WithSignals > {
        typed: TypedSigFinished < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigFinished < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigFinished < 'c, C > {
        type Target = TypedSigFinished < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigFinished < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for Tween {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfTween < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfTween < 'c, C > {
        type Target = < < Tween as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = Tween;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfTween < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = Tween;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
#![doc = "Sidecar module for class [`SpriteFrames`][crate::classes::SpriteFrames].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SpriteFrames` enums](https://docs.godotengine.org/en/stable/classes/class_spriteframes.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SpriteFrames`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`sprite_frames`][crate::classes::sprite_frames]: sidecar module with related enum/flag types\n* [`ISpriteFrames`][crate::classes::ISpriteFrames]: virtual methods\n\n\nSee also [Godot docs for `SpriteFrames`](https://docs.godotengine.org/en/stable/classes/class_spriteframes.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`SpriteFrames::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nSprite frame library for an [`AnimatedSprite2D`][crate::classes::AnimatedSprite2D] or [`AnimatedSprite3D`][crate::classes::AnimatedSprite3D] node. Contains frames and animation data for playback."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SpriteFrames {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`SpriteFrames`][crate::classes::SpriteFrames].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `SpriteFrames` methods](https://docs.godotengine.org/en/stable/classes/class_spriteframes.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISpriteFrames: crate::obj::GodotClass < Base = SpriteFrames > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl SpriteFrames {
        #[doc = "Adds a new `anim` animation to the library."]
        pub fn add_animation(&mut self, anim: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (anim.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8402usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteFrames", "add_animation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the `anim` animation exists."]
        pub fn has_animation(&self, anim: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (anim.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8403usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteFrames", "has_animation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Duplicates the animation `anim_from` to a new animation named `anim_to`. Fails if `anim_to` already exists, or if `anim_from` does not exist."]
        pub fn duplicate_animation(&mut self, anim_from: impl AsArg < StringName >, anim_to: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (anim_from.into_arg(), anim_to.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8404usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteFrames", "duplicate_animation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the `anim` animation."]
        pub fn remove_animation(&mut self, anim: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (anim.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8405usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteFrames", "remove_animation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Changes the `anim` animation's name to `newname`."]
        pub fn rename_animation(&mut self, anim: impl AsArg < StringName >, newname: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, StringName >,);
            let args = (anim.into_arg(), newname.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8406usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteFrames", "rename_animation", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns an array containing the names associated to each animation. Values are placed in alphabetical order."]
        pub fn get_animation_names(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8407usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteFrames", "get_animation_names", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the speed for the `anim` animation in frames per second."]
        pub fn set_animation_speed(&mut self, anim: impl AsArg < StringName >, fps: f64,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, f64,);
            let args = (anim.into_arg(), fps,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8408usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteFrames", "set_animation_speed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the speed in frames per second for the `anim` animation."]
        pub fn get_animation_speed(&self, anim: impl AsArg < StringName >,) -> f64 {
            type CallRet = f64;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (anim.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8409usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteFrames", "get_animation_speed", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `loop` is `true`, the `anim` animation will loop when it reaches the end, or the start if it is played in reverse."]
        pub fn set_animation_loop(&mut self, anim: impl AsArg < StringName >, loop_: bool,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, bool,);
            let args = (anim.into_arg(), loop_,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8410usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteFrames", "set_animation_loop", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the given animation is configured to loop when it finishes playing. Otherwise, returns `false`."]
        pub fn get_animation_loop(&self, anim: impl AsArg < StringName >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (anim.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8411usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteFrames", "get_animation_loop", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a frame to the `anim` animation. If `at_position` is `-1`, the frame will be added to the end of the animation. `duration` specifies the relative duration, see [`get_frame_duration`][`crate::classes::SpriteFrames::get_frame_duration`] for details."]
        pub(crate) fn add_frame_full(&mut self, anim: CowArg < StringName >, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >, duration: f32, at_position: i32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >, f32, i32,);
            let args = (anim, texture, duration, at_position,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8412usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteFrames", "add_frame", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_frame_ex`][Self::add_frame_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a frame to the `anim` animation. If `at_position` is `-1`, the frame will be added to the end of the animation. `duration` specifies the relative duration, see [`get_frame_duration`][`crate::classes::SpriteFrames::get_frame_duration`] for details."]
        #[inline]
        pub fn add_frame(&mut self, anim: impl AsArg < StringName >, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            self.add_frame_ex(anim, texture,) . done()
        }
        #[doc = "Adds a frame to the `anim` animation. If `at_position` is `-1`, the frame will be added to the end of the animation. `duration` specifies the relative duration, see [`get_frame_duration`][`crate::classes::SpriteFrames::get_frame_duration`] for details."]
        #[inline]
        pub fn add_frame_ex < 'ex > (&'ex mut self, anim: impl AsArg < StringName > + 'ex, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex,) -> ExAddFrame < 'ex > {
            ExAddFrame::new(self, anim, texture,)
        }
        #[doc = "Sets the `texture` and the `duration` of the frame `idx` in the `anim` animation. `duration` specifies the relative duration, see [`get_frame_duration`][`crate::classes::SpriteFrames::get_frame_duration`] for details."]
        pub(crate) fn set_frame_full(&mut self, anim: CowArg < StringName >, idx: i32, texture: CowArg < Option < Gd < crate::classes::Texture2D > > >, duration: f32,) {
            type CallRet = ();
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, StringName >, i32, CowArg < 'a1, Option < Gd < crate::classes::Texture2D > > >, f32,);
            let args = (anim, idx, texture, duration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8413usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteFrames", "set_frame", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`set_frame_ex`][Self::set_frame_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sets the `texture` and the `duration` of the frame `idx` in the `anim` animation. `duration` specifies the relative duration, see [`get_frame_duration`][`crate::classes::SpriteFrames::get_frame_duration`] for details."]
        #[inline]
        pub fn set_frame(&mut self, anim: impl AsArg < StringName >, idx: i32, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> >,) {
            self.set_frame_ex(anim, idx, texture,) . done()
        }
        #[doc = "Sets the `texture` and the `duration` of the frame `idx` in the `anim` animation. `duration` specifies the relative duration, see [`get_frame_duration`][`crate::classes::SpriteFrames::get_frame_duration`] for details."]
        #[inline]
        pub fn set_frame_ex < 'ex > (&'ex mut self, anim: impl AsArg < StringName > + 'ex, idx: i32, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex,) -> ExSetFrame < 'ex > {
            ExSetFrame::new(self, anim, idx, texture,)
        }
        #[doc = "Removes the `anim` animation's frame `idx`."]
        pub fn remove_frame(&mut self, anim: impl AsArg < StringName >, idx: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, i32,);
            let args = (anim.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8414usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteFrames", "remove_frame", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of frames for the `anim` animation."]
        pub fn get_frame_count(&self, anim: impl AsArg < StringName >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (anim.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8415usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteFrames", "get_frame_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the texture of the frame `idx` in the `anim` animation."]
        pub fn get_frame_texture(&self, anim: impl AsArg < StringName >, idx: i32,) -> Option < Gd < crate::classes::Texture2D > > {
            type CallRet = Option < Gd < crate::classes::Texture2D > >;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, i32,);
            let args = (anim.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8416usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteFrames", "get_frame_texture", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a relative duration of the frame `idx` in the `anim` animation (defaults to `1.0`). For example, a frame with a duration of `2.0` is displayed twice as long as a frame with a duration of `1.0`. You can calculate the absolute duration (in seconds) of a frame using the following formula:\n\n```gdscript\nabsolute_duration = relative_duration / (animation_fps * abs(playing_speed))\n```\n\nIn this example, `playing_speed` refers to either [`get_playing_speed`][`crate::classes::AnimatedSprite2D::get_playing_speed`] or [`get_playing_speed`][`crate::classes::AnimatedSprite3D::get_playing_speed`]."]
        pub fn get_frame_duration(&self, anim: impl AsArg < StringName >, idx: i32,) -> f32 {
            type CallRet = f32;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >, i32,);
            let args = (anim.into_arg(), idx,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8417usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteFrames", "get_frame_duration", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all frames from the `anim` animation."]
        pub fn clear(&mut self, anim: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (anim.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8418usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteFrames", "clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all animations. An empty `default` animation will be created."]
        pub fn clear_all(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8419usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SpriteFrames", "clear_all", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for SpriteFrames {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("SpriteFrames"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SpriteFrames {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for SpriteFrames {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for SpriteFrames {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SpriteFrames {
        
    }
    impl crate::obj::cap::GodotDefault for SpriteFrames {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SpriteFrames {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SpriteFrames {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`SpriteFrames`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_SpriteFrames__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::SpriteFrames > for $Class {
                
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
#[doc = "Default-param extender for [`SpriteFrames::add_frame_ex`][super::SpriteFrames::add_frame_ex]."]
#[must_use]
pub struct ExAddFrame < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::SpriteFrames, anim: CowArg < 'ex, StringName >, texture: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >, duration: f32, at_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddFrame < 'ex > {
    fn new(surround_object: &'ex mut re_export::SpriteFrames, anim: impl AsArg < StringName > + 'ex, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex,) -> Self {
        let duration = 1f32;
        let at_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, anim: anim.into_arg(), texture: texture.into_arg(), duration: duration, at_position: at_position,
        }
    }
    #[inline]
    pub fn duration(self, duration: f32) -> Self {
        Self {
            duration: duration, .. self
        }
    }
    #[inline]
    pub fn at_position(self, at_position: i32) -> Self {
        Self {
            at_position: at_position, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, anim, texture, duration, at_position,
        }
        = self;
        re_export::SpriteFrames::add_frame_full(surround_object, anim, texture, duration, at_position,)
    }
}
#[doc = "Default-param extender for [`SpriteFrames::set_frame_ex`][super::SpriteFrames::set_frame_ex]."]
#[must_use]
pub struct ExSetFrame < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::SpriteFrames, anim: CowArg < 'ex, StringName >, idx: i32, texture: CowArg < 'ex, Option < Gd < crate::classes::Texture2D > > >, duration: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSetFrame < 'ex > {
    fn new(surround_object: &'ex mut re_export::SpriteFrames, anim: impl AsArg < StringName > + 'ex, idx: i32, texture: impl AsArg < Option < Gd < crate::classes::Texture2D >> > + 'ex,) -> Self {
        let duration = 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, anim: anim.into_arg(), idx: idx, texture: texture.into_arg(), duration: duration,
        }
    }
    #[inline]
    pub fn duration(self, duration: f32) -> Self {
        Self {
            duration: duration, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, anim, idx, texture, duration,
        }
        = self;
        re_export::SpriteFrames::set_frame_full(surround_object, anim, idx, texture, duration,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::SpriteFrames;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for SpriteFrames {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfResource < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}
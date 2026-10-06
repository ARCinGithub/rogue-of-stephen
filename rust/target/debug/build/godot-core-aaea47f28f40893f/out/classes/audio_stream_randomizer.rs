#![doc = "Sidecar module for class [`AudioStreamRandomizer`][crate::classes::AudioStreamRandomizer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioStreamRandomizer` enums](https://docs.godotengine.org/en/stable/classes/class_audiostreamrandomizer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioStreamRandomizer`.\n\nInherits [`AudioStream`][crate::classes::AudioStream].\n\nRelated symbols:\n\n* [`audio_stream_randomizer`][crate::classes::audio_stream_randomizer]: sidecar module with related enum/flag types\n* [`IAudioStreamRandomizer`][crate::classes::IAudioStreamRandomizer]: virtual methods\n\n\nSee also [Godot docs for `AudioStreamRandomizer`](https://docs.godotengine.org/en/stable/classes/class_audiostreamrandomizer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AudioStreamRandomizer::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nPicks a random AudioStream from the pool, depending on the playback mode, and applies random pitch shifting and volume shifting during playback."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioStreamRandomizer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AudioStreamRandomizer`][crate::classes::AudioStreamRandomizer].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IAudioStream`][crate::classes::IAudioStream] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AudioStreamRandomizer` methods](https://docs.godotengine.org/en/stable/classes/class_audiostreamrandomizer.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioStreamRandomizer: crate::obj::GodotClass < Base = AudioStreamRandomizer > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Override this method to customize the returned value of [`instantiate_playback`][`crate::classes::AudioStream::instantiate_playback`]. Should return a new [`AudioStreamPlayback`][crate::classes::AudioStreamPlayback] created when the stream is played (such as by an [`AudioStreamPlayer`][crate::classes::AudioStreamPlayer])."]
        fn instantiate_playback(&self,) -> Option < Gd < crate::classes::AudioStreamPlayback > >;
        #[doc = "Override this method to customize the name assigned to this audio stream. Unused by the engine."]
        fn get_stream_name(&self,) -> GString {
            unimplemented !()
        }
        #[doc = "Override this method to customize the returned value of [`get_length`][`crate::classes::AudioStream::get_length`]. Should return the length of this audio stream, in seconds."]
        fn get_length(&self,) -> f64 {
            unimplemented !()
        }
        #[doc = "Override this method to customize the returned value of [`is_monophonic`][`crate::classes::AudioStream::is_monophonic`]. Should return `true` if this audio stream only supports one channel."]
        fn is_monophonic(&self,) -> bool {
            unimplemented !()
        }
        #[doc = "Overridable method. Should return the tempo of this audio stream, in beats per minute (BPM). Used by the engine to determine the position of every beat.\n\nIdeally, the returned value should be based off the stream's sample rate (\\[member AudioStreamWAV.mix_rate], for example)."]
        fn get_bpm(&self,) -> f64 {
            unimplemented !()
        }
        #[doc = "Overridable method. Should return the total number of beats of this audio stream. Used by the engine to determine the position of every beat.\n\nIdeally, the returned value should be based off the stream's sample rate (\\[member AudioStreamWAV.mix_rate], for example)."]
        fn get_beat_count(&self,) -> i32 {
            unimplemented !()
        }
        #[doc = "Override this method to customize the tags for this audio stream. Should return a [`Dictionary`][crate::builtin::Dictionary] of strings with the tag as the key and its content as the value.\n\nCommonly used tags include `title`, `artist`, `album`, `tracknumber`, and `date`."]
        fn get_tags(&self,) -> AnyDictionary {
            unimplemented !()
        }
        #[doc = "Return the controllable parameters of this stream. This array contains dictionaries with a property info description format (see [`get_property_list`][`crate::classes::Object::get_property_list`]). Additionally, the default value for this parameter must be added tho each dictionary in \"default_value\" field."]
        fn get_parameter_list(&self,) -> Array < AnyDictionary > {
            unimplemented !()
        }
        #[doc = "Override this method to return `true` if this stream has a loop."]
        fn has_loop(&self,) -> bool {
            unimplemented !()
        }
        #[doc = "Override this method to return the bar beats of this stream."]
        fn get_bar_beats(&self,) -> i32 {
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
    impl AudioStreamRandomizer {
        #[doc = "Insert a stream at the specified index. If the index is less than zero, the insertion occurs at the end of the underlying pool."]
        pub(crate) fn add_stream_full(&mut self, index: i32, stream: CowArg < Option < Gd < crate::classes::AudioStream > > >, weight: f32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::AudioStream > > >, f32,);
            let args = (index, stream, weight,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10490usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "add_stream", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_stream_ex`][Self::add_stream_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Insert a stream at the specified index. If the index is less than zero, the insertion occurs at the end of the underlying pool."]
        #[inline]
        pub fn add_stream(&mut self, index: i32, stream: impl AsArg < Option < Gd < crate::classes::AudioStream >> >,) {
            self.add_stream_ex(index, stream,) . done()
        }
        #[doc = "Insert a stream at the specified index. If the index is less than zero, the insertion occurs at the end of the underlying pool."]
        #[inline]
        pub fn add_stream_ex < 'ex > (&'ex mut self, index: i32, stream: impl AsArg < Option < Gd < crate::classes::AudioStream >> > + 'ex,) -> ExAddStream < 'ex > {
            ExAddStream::new(self, index, stream,)
        }
        #[doc = "Move a stream from one index to another."]
        pub fn move_stream(&mut self, index_from: i32, index_to: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (index_from, index_to,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10491usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "move_stream", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Remove the stream at the specified index."]
        pub fn remove_stream(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10492usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "remove_stream", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Set the AudioStream at the specified index."]
        pub fn set_stream(&mut self, index: i32, stream: impl AsArg < Option < Gd < crate::classes::AudioStream >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::AudioStream > > >,);
            let args = (index, stream.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10493usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "set_stream", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the stream at the specified index."]
        pub fn get_stream(&self, index: i32,) -> Option < Gd < crate::classes::AudioStream > > {
            type CallRet = Option < Gd < crate::classes::AudioStream > >;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10494usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "get_stream", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Set the probability weight of the stream at the specified index. The higher this value, the more likely that the randomizer will choose this stream during random playback modes."]
        pub fn set_stream_probability_weight(&mut self, index: i32, weight: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (index, weight,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10495usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "set_stream_probability_weight", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the probability weight associated with the stream at the given index."]
        pub fn get_stream_probability_weight(&self, index: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10496usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "get_stream_probability_weight", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_streams_count(&mut self, count: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (count,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10497usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "set_streams_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_streams_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10498usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "get_streams_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_random_pitch(&mut self, scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10499usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "set_random_pitch", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_random_pitch(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10500usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "get_random_pitch", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_random_pitch_semitones(&mut self, semitones: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (semitones,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10501usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "set_random_pitch_semitones", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_random_pitch_semitones(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10502usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "get_random_pitch_semitones", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_random_volume_offset_db(&mut self, db_offset: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (db_offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10503usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "set_random_volume_offset_db", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_random_volume_offset_db(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10504usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "get_random_volume_offset_db", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_playback_mode(&mut self, mode: crate::classes::audio_stream_randomizer::PlaybackMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::audio_stream_randomizer::PlaybackMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10505usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "set_playback_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_playback_mode(&self,) -> crate::classes::audio_stream_randomizer::PlaybackMode {
            type CallRet = crate::classes::audio_stream_randomizer::PlaybackMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10506usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamRandomizer", "get_playback_mode", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AudioStreamRandomizer {
        type Base = crate::classes::AudioStream;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AudioStreamRandomizer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioStreamRandomizer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AudioStream > for AudioStreamRandomizer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AudioStreamRandomizer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AudioStreamRandomizer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioStreamRandomizer {
        
    }
    impl crate::obj::cap::GodotDefault for AudioStreamRandomizer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioStreamRandomizer {
        type Target = crate::classes::AudioStream;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioStreamRandomizer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AudioStreamRandomizer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AudioStreamRandomizer__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioStreamRandomizer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioStream > for $Class {
                
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
#[doc = "Default-param extender for [`AudioStreamRandomizer::add_stream_ex`][super::AudioStreamRandomizer::add_stream_ex]."]
#[must_use]
pub struct ExAddStream < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AudioStreamRandomizer, index: i32, stream: CowArg < 'ex, Option < Gd < crate::classes::AudioStream > > >, weight: f32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddStream < 'ex > {
    fn new(surround_object: &'ex mut re_export::AudioStreamRandomizer, index: i32, stream: impl AsArg < Option < Gd < crate::classes::AudioStream >> > + 'ex,) -> Self {
        let weight = 1f32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, index: index, stream: stream.into_arg(), weight: weight,
        }
    }
    #[inline]
    pub fn weight(self, weight: f32) -> Self {
        Self {
            weight: weight, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, index, stream, weight,
        }
        = self;
        re_export::AudioStreamRandomizer::add_stream_full(surround_object, index, stream, weight,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PlaybackMode {
    ord: i32
}
impl PlaybackMode {
    #[doc(alias = "PLAYBACK_RANDOM_NO_REPEATS")]
    #[doc = "Godot enumerator name: `PLAYBACK_RANDOM_NO_REPEATS`"]
    pub const RANDOM_NO_REPEATS: PlaybackMode = PlaybackMode {
        ord: 0i32
    };
    #[doc(alias = "PLAYBACK_RANDOM")]
    #[doc = "Godot enumerator name: `PLAYBACK_RANDOM`"]
    pub const RANDOM: PlaybackMode = PlaybackMode {
        ord: 1i32
    };
    #[doc(alias = "PLAYBACK_SEQUENTIAL")]
    #[doc = "Godot enumerator name: `PLAYBACK_SEQUENTIAL`"]
    pub const SEQUENTIAL: PlaybackMode = PlaybackMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for PlaybackMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PlaybackMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PlaybackMode {
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
            Self::RANDOM_NO_REPEATS => "RANDOM_NO_REPEATS", Self::RANDOM => "RANDOM", Self::SEQUENTIAL => "SEQUENTIAL", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PlaybackMode::RANDOM_NO_REPEATS, PlaybackMode::RANDOM, PlaybackMode::SEQUENTIAL]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PlaybackMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("RANDOM_NO_REPEATS", "PLAYBACK_RANDOM_NO_REPEATS", PlaybackMode::RANDOM_NO_REPEATS), crate::meta::inspect::EnumConstant::new("RANDOM", "PLAYBACK_RANDOM", PlaybackMode::RANDOM), crate::meta::inspect::EnumConstant::new("SEQUENTIAL", "PLAYBACK_SEQUENTIAL", PlaybackMode::SEQUENTIAL)]
        }
    }
}
impl crate::meta::GodotConvert for PlaybackMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Playback Random No Repeats", 0i64), EnumeratorShape::new_int("Playback Random", 1i64), EnumeratorShape::new_int("Playback Sequential", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("AudioStreamRandomizer.PlaybackMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for PlaybackMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PlaybackMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for PlaybackMode {
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
impl crate::registry::property::Export for PlaybackMode {
    
}
impl crate::meta::Element for PlaybackMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AudioStreamRandomizer;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::audio_stream::SignalsOfAudioStream;
    impl WithSignals for AudioStreamRandomizer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfAudioStream < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}
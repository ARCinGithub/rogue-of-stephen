#![doc = "Sidecar module for class [`AudioStream`][crate::classes::AudioStream].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioStream` enums](https://docs.godotengine.org/en/stable/classes/class_audiostream.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioStream`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`audio_stream`][crate::classes::audio_stream]: sidecar module with related enum/flag types\n* [`IAudioStream`][crate::classes::IAudioStream]: virtual methods\n* [`SignalsOfAudioStream`][crate::classes::audio_stream::SignalsOfAudioStream]: signal collection\n\n\nSee also [Godot docs for `AudioStream`](https://docs.godotengine.org/en/stable/classes/class_audiostream.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AudioStream::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nBase class for audio streams. Audio streams are used for sound effects and music playback, and support WAV (via [`AudioStreamWAV`][crate::classes::AudioStreamWav]) and Ogg (via [`AudioStreamOggVorbis`][crate::classes::AudioStreamOggVorbis]) file formats."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioStream {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AudioStream`][crate::classes::AudioStream].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AudioStream` methods](https://docs.godotengine.org/en/stable/classes/class_audiostream.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioStream: crate::obj::GodotClass < Base = AudioStream > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AudioStream {
        #[doc = "Returns the length of the audio stream in seconds. If this stream is an [`AudioStreamRandomizer`][crate::classes::AudioStreamRandomizer], returns the length of the last played stream. If this stream has an indefinite length (such as for [`AudioStreamGenerator`][crate::classes::AudioStreamGenerator] and [`AudioStreamMicrophone`][crate::classes::AudioStreamMicrophone]), returns `0.0`."]
        pub fn get_length(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7004usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStream", "get_length", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if this audio stream only supports one channel (_monophony_), or `false` if the audio stream supports two or more channels (_polyphony_)."]
        pub fn is_monophonic(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7005usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStream", "is_monophonic", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a newly created [`AudioStreamPlayback`][crate::classes::AudioStreamPlayback] intended to play this audio stream. Useful for when you want to extend [`instantiate_playback`][`crate::classes::IAudioStream::instantiate_playback`] but call [`instantiate_playback`][`crate::classes::AudioStream::instantiate_playback`] from an internally held AudioStream subresource. An example of this can be found in the source code for `AudioStreamRandomPitch::instantiate_playback`."]
        pub fn instantiate_playback(&mut self,) -> Option < Gd < crate::classes::AudioStreamPlayback > > {
            type CallRet = Option < Gd < crate::classes::AudioStreamPlayback > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7006usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStream", "instantiate_playback", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns if the current `AudioStream` can be used as a sample. Only static streams can be sampled."]
        pub fn can_be_sampled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7007usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStream", "can_be_sampled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the stream is a collection of other streams, `false` otherwise."]
        pub fn is_meta_stream(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7008usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStream", "is_meta_stream", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AudioStream {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AudioStream"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioStream {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AudioStream {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AudioStream {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioStream {
        
    }
    impl crate::obj::cap::GodotDefault for AudioStream {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioStream {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioStream {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AudioStream`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AudioStream__ensure_class_exists {
        ($Class: ident) => {
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
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AudioStream;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`AudioStream`][crate::classes::AudioStream] class."]
    pub struct SignalsOfAudioStream < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfAudioStream < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn parameter_list_changed(&mut self) -> SigParameterListChanged < 'c, C > {
            SigParameterListChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "parameter_list_changed")
            }
        }
    }
    type TypedSigParameterListChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigParameterListChanged < 'c, C: WithSignals > {
        typed: TypedSigParameterListChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigParameterListChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigParameterListChanged < 'c, C > {
        type Target = TypedSigParameterListChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigParameterListChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for AudioStream {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfAudioStream < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfAudioStream < 'c, C > {
        type Target = < < AudioStream as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = AudioStream;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfAudioStream < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = AudioStream;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
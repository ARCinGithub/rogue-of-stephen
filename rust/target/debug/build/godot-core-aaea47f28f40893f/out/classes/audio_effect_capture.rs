#![doc = "Sidecar module for class [`AudioEffectCapture`][crate::classes::AudioEffectCapture].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioEffectCapture` enums](https://docs.godotengine.org/en/stable/classes/class_audioeffectcapture.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioEffectCapture`.\n\nInherits [`AudioEffect`][crate::classes::AudioEffect].\n\nRelated symbols:\n\n* [`IAudioEffectCapture`][crate::classes::IAudioEffectCapture]: virtual methods\n\n\nSee also [Godot docs for `AudioEffectCapture`](https://docs.godotengine.org/en/stable/classes/class_audioeffectcapture.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AudioEffectCapture::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nAudioEffectCapture is an AudioEffect which copies all audio frames from the attached audio effect bus into its internal ring buffer.\n\nApplication code should consume these audio frames from this ring buffer using [`get_buffer`][`crate::classes::AudioEffectCapture::get_buffer`] and process it as needed, for example to capture data from an [`AudioStreamMicrophone`][crate::classes::AudioStreamMicrophone], implement application-defined effects, or to transmit audio over the network. When capturing audio data from a microphone, the format of the samples will be stereo 32-bit floating-point PCM.\n\nUnlike [`AudioEffectRecord`][crate::classes::AudioEffectRecord], this effect only returns the raw audio samples instead of encoding them into an [`AudioStream`][crate::classes::AudioStream]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioEffectCapture {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AudioEffectCapture`][crate::classes::AudioEffectCapture].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IAudioEffect`][crate::classes::IAudioEffect] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AudioEffectCapture` methods](https://docs.godotengine.org/en/stable/classes/class_audioeffectcapture.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioEffectCapture: crate::obj::GodotClass < Base = AudioEffectCapture > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Override this method to customize the [`AudioEffectInstance`][crate::classes::AudioEffectInstance] created when this effect is applied on a bus in the editor's Audio panel, or through [`add_bus_effect`][`crate::classes::AudioServer::add_bus_effect`].\n\n```gdscript\nextends AudioEffect\n\n@export var strength = 4.0\n\nfunc _instantiate():\n\tvar effect = CustomAudioEffectInstance.new()\n\teffect.base = self\n\n\treturn effect\n```\n\n**Note:** It is recommended to keep a reference to the original `AudioEffect` in the new instance. Depending on the implementation this allows the effect instance to listen for changes at run-time and be modified accordingly."]
        fn instantiate(&mut self,) -> Option < Gd < crate::classes::AudioEffectInstance > >;
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
    impl AudioEffectCapture {
        #[doc = "Returns `true` if at least `frames` audio frames are available to read in the internal ring buffer."]
        pub fn can_get_buffer(&self, frames: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (frames,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6835usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectCapture", "can_get_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Gets the next `frames` audio samples from the internal ring buffer.\n\nReturns a [`PackedVector2Array`][crate::builtin::PackedVector2Array] containing exactly `frames` audio samples if available, or an empty [`PackedVector2Array`][crate::builtin::PackedVector2Array] if insufficient data was available.\n\nThe samples are signed floating-point PCM between `-1` and `1`. You will have to scale them if you want to use them as 8 or 16-bit integer samples. (`v = 0x7fff * samples[0].x`)"]
        pub fn get_buffer(&self, frames: i32,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams = (i32,);
            let args = (frames,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6836usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectCapture", "get_buffer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears the internal ring buffer.\n\n**Note:** Calling this during a capture can cause the loss of samples which causes popping in the playback."]
        pub fn clear_buffer(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6837usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectCapture", "clear_buffer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_buffer_length(&mut self, buffer_length_seconds: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (buffer_length_seconds,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6838usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectCapture", "set_buffer_length", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_buffer_length(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6839usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectCapture", "get_buffer_length", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of frames available to read using [`get_buffer`][`crate::classes::AudioEffectCapture::get_buffer`]."]
        pub fn get_frames_available(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6840usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectCapture", "get_frames_available", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of audio frames discarded from the audio bus due to full buffer."]
        pub fn get_discarded_frames(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6841usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectCapture", "get_discarded_frames", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the total size of the internal ring buffer in frames."]
        pub fn get_buffer_length_frames(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6842usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectCapture", "get_buffer_length_frames", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of audio frames inserted from the audio bus."]
        pub fn get_pushed_frames(&self,) -> i64 {
            type CallRet = i64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6843usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectCapture", "get_pushed_frames", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AudioEffectCapture {
        type Base = crate::classes::AudioEffect;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AudioEffectCapture"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioEffectCapture {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AudioEffect > for AudioEffectCapture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AudioEffectCapture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AudioEffectCapture {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioEffectCapture {
        
    }
    impl crate::obj::cap::GodotDefault for AudioEffectCapture {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioEffectCapture {
        type Target = crate::classes::AudioEffect;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioEffectCapture {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AudioEffectCapture`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AudioEffectCapture__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioEffectCapture > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioEffect > for $Class {
                
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
    use super::re_export::AudioEffectCapture;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for AudioEffectCapture {
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
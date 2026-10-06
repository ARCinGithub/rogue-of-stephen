#![doc = "Sidecar module for class [`AudioStreamPlaybackPolyphonic`][crate::classes::AudioStreamPlaybackPolyphonic].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioStreamPlaybackPolyphonic` enums](https://docs.godotengine.org/en/stable/classes/class_audiostreamplaybackpolyphonic.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioStreamPlaybackPolyphonic`.\n\nInherits [`AudioStreamPlayback`][crate::classes::AudioStreamPlayback].\n\nRelated symbols:\n\n* [`audio_stream_playback_polyphonic`][crate::classes::audio_stream_playback_polyphonic]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `AudioStreamPlaybackPolyphonic`](https://docs.godotengine.org/en/stable/classes/class_audiostreamplaybackpolyphonic.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<AudioStreamPlaybackPolyphonic>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nPlayback instance for [`AudioStreamPolyphonic`][crate::classes::AudioStreamPolyphonic]. After setting the `stream` property of [`AudioStreamPlayer`][crate::classes::AudioStreamPlayer], [`AudioStreamPlayer2D`][crate::classes::AudioStreamPlayer2D], or [`AudioStreamPlayer3D`][crate::classes::AudioStreamPlayer3D], the playback instance can be obtained by calling [`get_stream_playback`][`crate::classes::AudioStreamPlayer::get_stream_playback`], [`get_stream_playback`][`crate::classes::AudioStreamPlayer2D::get_stream_playback`] or [`get_stream_playback`][`crate::classes::AudioStreamPlayer3D::get_stream_playback`] methods."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioStreamPlaybackPolyphonic {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl AudioStreamPlaybackPolyphonic {
        #[doc = "Play an [`AudioStream`][crate::classes::AudioStream] at a given offset, volume, pitch scale, playback type, and bus. Playback starts immediately.\n\nThe return value is a unique integer ID that is associated to this playback stream and which can be used to control it.\n\nThis ID becomes invalid when the stream ends (if it does not loop), when the `AudioStreamPlaybackPolyphonic` is stopped, or when [`stop_stream`][`crate::classes::AudioStreamPlaybackPolyphonic::stop_stream`] is called.\n\nThis function returns `INVALID_ID` if the amount of streams currently playing equals \\[member AudioStreamPolyphonic.polyphony]. If you need a higher amount of maximum polyphony, raise this value."]
        pub(crate) fn play_stream_full(&mut self, stream: CowArg < Option < Gd < crate::classes::AudioStream > > >, from_offset: f32, volume_db: f32, pitch_scale: f32, playback_type: crate::classes::audio_server::PlaybackType, bus: CowArg < StringName >,) -> i64 {
            type CallRet = i64;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::AudioStream > > >, f32, f32, f32, crate::classes::audio_server::PlaybackType, CowArg < 'a1, StringName >,);
            let args = (stream, from_offset, volume_db, pitch_scale, playback_type, bus,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10507usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlaybackPolyphonic", "play_stream", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`play_stream_ex`][Self::play_stream_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Play an [`AudioStream`][crate::classes::AudioStream] at a given offset, volume, pitch scale, playback type, and bus. Playback starts immediately.\n\nThe return value is a unique integer ID that is associated to this playback stream and which can be used to control it.\n\nThis ID becomes invalid when the stream ends (if it does not loop), when the `AudioStreamPlaybackPolyphonic` is stopped, or when [`stop_stream`][`crate::classes::AudioStreamPlaybackPolyphonic::stop_stream`] is called.\n\nThis function returns `INVALID_ID` if the amount of streams currently playing equals \\[member AudioStreamPolyphonic.polyphony]. If you need a higher amount of maximum polyphony, raise this value."]
        #[inline]
        pub fn play_stream(&mut self, stream: impl AsArg < Option < Gd < crate::classes::AudioStream >> >,) -> i64 {
            self.play_stream_ex(stream,) . done()
        }
        #[doc = "Play an [`AudioStream`][crate::classes::AudioStream] at a given offset, volume, pitch scale, playback type, and bus. Playback starts immediately.\n\nThe return value is a unique integer ID that is associated to this playback stream and which can be used to control it.\n\nThis ID becomes invalid when the stream ends (if it does not loop), when the `AudioStreamPlaybackPolyphonic` is stopped, or when [`stop_stream`][`crate::classes::AudioStreamPlaybackPolyphonic::stop_stream`] is called.\n\nThis function returns `INVALID_ID` if the amount of streams currently playing equals \\[member AudioStreamPolyphonic.polyphony]. If you need a higher amount of maximum polyphony, raise this value."]
        #[inline]
        pub fn play_stream_ex < 'ex > (&'ex mut self, stream: impl AsArg < Option < Gd < crate::classes::AudioStream >> > + 'ex,) -> ExPlayStream < 'ex > {
            ExPlayStream::new(self, stream,)
        }
        #[doc = "Change the stream volume (in db). The `stream` argument is an integer ID returned by [`play_stream`][`crate::classes::AudioStreamPlaybackPolyphonic::play_stream`]."]
        pub fn set_stream_volume(&mut self, stream: i64, volume_db: f32,) {
            type CallRet = ();
            type CallParams = (i64, f32,);
            let args = (stream, volume_db,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10508usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlaybackPolyphonic", "set_stream_volume", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Change the stream pitch scale. The `stream` argument is an integer ID returned by [`play_stream`][`crate::classes::AudioStreamPlaybackPolyphonic::play_stream`]."]
        pub fn set_stream_pitch_scale(&mut self, stream: i64, pitch_scale: f32,) {
            type CallRet = ();
            type CallParams = (i64, f32,);
            let args = (stream, pitch_scale,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10509usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlaybackPolyphonic", "set_stream_pitch_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the stream associated with the given integer ID is still playing. Check [`play_stream`][`crate::classes::AudioStreamPlaybackPolyphonic::play_stream`] for information on when this ID becomes invalid."]
        pub fn is_stream_playing(&self, stream: i64,) -> bool {
            type CallRet = bool;
            type CallParams = (i64,);
            let args = (stream,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10510usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlaybackPolyphonic", "is_stream_playing", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Stop a stream. The `stream` argument is an integer ID returned by [`play_stream`][`crate::classes::AudioStreamPlaybackPolyphonic::play_stream`], which becomes invalid after calling this function."]
        pub fn stop_stream(&mut self, stream: i64,) {
            type CallRet = ();
            type CallParams = (i64,);
            let args = (stream,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10511usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamPlaybackPolyphonic", "stop_stream", Some(self.__validated_obj()), args,)
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
        pub const INVALID_ID: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for AudioStreamPlaybackPolyphonic {
        type Base = crate::classes::AudioStreamPlayback;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AudioStreamPlaybackPolyphonic"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioStreamPlaybackPolyphonic {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AudioStreamPlayback > for AudioStreamPlaybackPolyphonic {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AudioStreamPlaybackPolyphonic {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioStreamPlaybackPolyphonic {
        
    }
    impl std::ops::Deref for AudioStreamPlaybackPolyphonic {
        type Target = crate::classes::AudioStreamPlayback;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioStreamPlaybackPolyphonic {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AudioStreamPlaybackPolyphonic__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `AudioStreamPlaybackPolyphonic` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`AudioStreamPlaybackPolyphonic::play_stream_ex`][super::AudioStreamPlaybackPolyphonic::play_stream_ex]."]
#[must_use]
pub struct ExPlayStream < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AudioStreamPlaybackPolyphonic, stream: CowArg < 'ex, Option < Gd < crate::classes::AudioStream > > >, from_offset: f32, volume_db: f32, pitch_scale: f32, playback_type: crate::classes::audio_server::PlaybackType, bus: CowArg < 'ex, StringName >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExPlayStream < 'ex > {
    fn new(surround_object: &'ex mut re_export::AudioStreamPlaybackPolyphonic, stream: impl AsArg < Option < Gd < crate::classes::AudioStream >> > + 'ex,) -> Self {
        let from_offset = 0f32;
        let volume_db = 0f32;
        let pitch_scale = 1f32;
        let playback_type = crate::obj::EngineEnum::from_ord(0);
        let bus = StringName::from("Master");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, stream: stream.into_arg(), from_offset: from_offset, volume_db: volume_db, pitch_scale: pitch_scale, playback_type: playback_type, bus: CowArg::Owned(bus),
        }
    }
    #[inline]
    pub fn from_offset(self, from_offset: f32) -> Self {
        Self {
            from_offset: from_offset, .. self
        }
    }
    #[inline]
    pub fn volume_db(self, volume_db: f32) -> Self {
        Self {
            volume_db: volume_db, .. self
        }
    }
    #[inline]
    pub fn pitch_scale(self, pitch_scale: f32) -> Self {
        Self {
            pitch_scale: pitch_scale, .. self
        }
    }
    #[inline]
    pub fn playback_type(self, playback_type: crate::classes::audio_server::PlaybackType) -> Self {
        Self {
            playback_type: playback_type, .. self
        }
    }
    #[inline]
    pub fn bus(self, bus: impl AsArg < StringName > + 'ex) -> Self {
        Self {
            bus: bus.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> i64 {
        let Self {
            _phantom, surround_object, stream, from_offset, volume_db, pitch_scale, playback_type, bus,
        }
        = self;
        re_export::AudioStreamPlaybackPolyphonic::play_stream_full(surround_object, stream, from_offset, volume_db, pitch_scale, playback_type, bus,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AudioStreamPlaybackPolyphonic;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for AudioStreamPlaybackPolyphonic {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfObject < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}
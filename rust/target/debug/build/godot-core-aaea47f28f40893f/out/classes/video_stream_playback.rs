#![doc = "Sidecar module for class [`VideoStreamPlayback`][crate::classes::VideoStreamPlayback].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VideoStreamPlayback` enums](https://docs.godotengine.org/en/stable/classes/class_videostreamplayback.html#enumerations).\n\n"]
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
    #[doc = "Godot class `VideoStreamPlayback`.\n\nInherits [`Resource`][crate::classes::Resource].\n\nRelated symbols:\n\n* [`video_stream_playback`][crate::classes::video_stream_playback]: sidecar module with related enum/flag types\n* [`IVideoStreamPlayback`][crate::classes::IVideoStreamPlayback]: virtual methods\n\n\nSee also [Godot docs for `VideoStreamPlayback`](https://docs.godotengine.org/en/stable/classes/class_videostreamplayback.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`VideoStreamPlayback::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nThis class is intended to be overridden by video decoder extensions with custom implementations of [`VideoStream`][crate::classes::VideoStream]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct VideoStreamPlayback {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`VideoStreamPlayback`][crate::classes::VideoStreamPlayback].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `VideoStreamPlayback` methods](https://docs.godotengine.org/en/stable/classes/class_videostreamplayback.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IVideoStreamPlayback: crate::obj::GodotClass < Base = VideoStreamPlayback > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "Stops playback. May be called multiple times before [`play`][`crate::classes::IVideoStreamPlayback::play`], or in response to [`stop`][`crate::classes::VideoStreamPlayer::stop`]. [`is_playing`][`crate::classes::IVideoStreamPlayback::is_playing`] should return `false` once stopped."]
        fn stop(&mut self,) {
            unimplemented !()
        }
        #[doc = "Called in response to \\[member VideoStreamPlayer.autoplay] or [`play`][`crate::classes::VideoStreamPlayer::play`]. Note that manual playback may also invoke [`stop`][`crate::classes::IVideoStreamPlayback::stop`] multiple times before this method is called. [`is_playing`][`crate::classes::IVideoStreamPlayback::is_playing`] should return `true` once playing."]
        fn play(&mut self,) {
            unimplemented !()
        }
        #[doc = "Returns the playback state, as determined by calls to [`play`][`crate::classes::IVideoStreamPlayback::play`] and [`stop`][`crate::classes::IVideoStreamPlayback::stop`]."]
        fn is_playing(&self,) -> bool {
            unimplemented !()
        }
        #[doc = "Set the paused status of video playback. [`is_paused`][`crate::classes::IVideoStreamPlayback::is_paused`] must return `paused`. Called in response to the \\[member VideoStreamPlayer.paused] setter."]
        fn set_paused(&mut self, paused: bool,) {
            unimplemented !()
        }
        #[doc = "Returns the paused status, as set by [`set_paused`][`crate::classes::IVideoStreamPlayback::set_paused`]."]
        fn is_paused(&self,) -> bool {
            unimplemented !()
        }
        #[doc = "Returns the video duration in seconds, if known, or 0 if unknown."]
        fn get_length(&self,) -> f64 {
            unimplemented !()
        }
        #[doc = "Return the current playback timestamp. Called in response to the \\[member VideoStreamPlayer.stream_position] getter."]
        fn get_playback_position(&self,) -> f64 {
            unimplemented !()
        }
        #[doc = "Seeks to `time` seconds. Called in response to the \\[member VideoStreamPlayer.stream_position] setter."]
        fn seek(&mut self, time: f64,) {
            unimplemented !()
        }
        #[doc = "Select the audio track `idx`. Called when playback starts, and in response to the \\[member VideoStreamPlayer.audio_track] setter."]
        fn set_audio_track(&mut self, idx: i32,) {
            unimplemented !()
        }
        #[doc = "Allocates a [`Texture2D`][crate::classes::Texture2D] in which decoded video frames will be drawn."]
        fn get_texture(&self,) -> Option < Gd < crate::classes::Texture2D > > {
            unimplemented !()
        }
        #[doc = "Ticks video playback for `delta` seconds. Called every frame as long as both [`is_paused`][`crate::classes::IVideoStreamPlayback::is_paused`] and [`is_playing`][`crate::classes::IVideoStreamPlayback::is_playing`] return `true`."]
        fn update(&mut self, delta: f64,);
        #[doc = "Returns the number of audio channels."]
        fn get_channels(&self,) -> i32 {
            unimplemented !()
        }
        #[doc = "Returns the audio sample rate used for mixing."]
        fn get_mix_rate(&self,) -> i32 {
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
    impl VideoStreamPlayback {
        #[doc = "Render `num_frames` audio frames (of [`get_channels`][`crate::classes::IVideoStreamPlayback::get_channels`] floats each) from `buffer`, starting from index `offset` in the array. Returns the number of audio frames rendered, or -1 on error."]
        pub(crate) fn mix_audio_full(&mut self, num_frames: i32, buffer: RefArg < PackedFloat32Array >, offset: i32,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (i32, RefArg < 'a0, PackedFloat32Array >, i32,);
            let args = (num_frames, buffer, offset,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7282usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "VideoStreamPlayback", "mix_audio", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`mix_audio_ex`][Self::mix_audio_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Render `num_frames` audio frames (of [`get_channels`][`crate::classes::IVideoStreamPlayback::get_channels`] floats each) from `buffer`, starting from index `offset` in the array. Returns the number of audio frames rendered, or -1 on error."]
        #[inline]
        pub fn mix_audio(&mut self, num_frames: i32,) -> i32 {
            self.mix_audio_ex(num_frames,) . done()
        }
        #[doc = "Render `num_frames` audio frames (of [`get_channels`][`crate::classes::IVideoStreamPlayback::get_channels`] floats each) from `buffer`, starting from index `offset` in the array. Returns the number of audio frames rendered, or -1 on error."]
        #[inline]
        pub fn mix_audio_ex < 'ex > (&'ex mut self, num_frames: i32,) -> ExMixAudio < 'ex > {
            ExMixAudio::new(self, num_frames,)
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
    impl crate::obj::GodotClass for VideoStreamPlayback {
        type Base = crate::classes::Resource;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("VideoStreamPlayback"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for VideoStreamPlayback {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for VideoStreamPlayback {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for VideoStreamPlayback {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for VideoStreamPlayback {
        
    }
    impl crate::obj::cap::GodotDefault for VideoStreamPlayback {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for VideoStreamPlayback {
        type Target = crate::classes::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for VideoStreamPlayback {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`VideoStreamPlayback`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_VideoStreamPlayback__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::VideoStreamPlayback > for $Class {
                
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
#[doc = "Default-param extender for [`VideoStreamPlayback::mix_audio_ex`][super::VideoStreamPlayback::mix_audio_ex]."]
#[must_use]
pub struct ExMixAudio < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::VideoStreamPlayback, num_frames: i32, buffer: CowArg < 'ex, PackedFloat32Array >, offset: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExMixAudio < 'ex > {
    fn new(surround_object: &'ex mut re_export::VideoStreamPlayback, num_frames: i32,) -> Self {
        let buffer = PackedFloat32Array::new();
        let offset = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, num_frames: num_frames, buffer: CowArg::Owned(buffer), offset: offset,
        }
    }
    #[inline]
    pub fn buffer(self, buffer: &'ex PackedFloat32Array) -> Self {
        Self {
            buffer: CowArg::Borrowed(buffer), .. self
        }
    }
    #[inline]
    pub fn offset(self, offset: i32) -> Self {
        Self {
            offset: offset, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, num_frames, buffer, offset,
        }
        = self;
        re_export::VideoStreamPlayback::mix_audio_full(surround_object, num_frames, buffer.cow_as_arg(), offset,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::VideoStreamPlayback;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for VideoStreamPlayback {
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
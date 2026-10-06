#![doc = "Sidecar module for class [`AudioStreamWav`][crate::classes::AudioStreamWav].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioStreamWAV` enums](https://docs.godotengine.org/en/stable/classes/class_audiostreamwav.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioStreamWAV`.\n\nInherits [`AudioStream`][crate::classes::AudioStream].\n\nRelated symbols:\n\n* [`audio_stream_wav`][crate::classes::audio_stream_wav]: sidecar module with related enum/flag types\n* [`IAudioStreamWav`][crate::classes::IAudioStreamWav]: virtual methods\n\n\nSee also [Godot docs for `AudioStreamWAV`](https://docs.godotengine.org/en/stable/classes/class_audiostreamwav.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AudioStreamWav::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nAudioStreamWAV stores sound samples loaded from WAV files. To play the stored sound, use an [`AudioStreamPlayer`][crate::classes::AudioStreamPlayer] (for non-positional audio) or [`AudioStreamPlayer2D`][crate::classes::AudioStreamPlayer2D]/[`AudioStreamPlayer3D`][crate::classes::AudioStreamPlayer3D] (for positional audio). The sound can be looped.\n\nThis class can also be used to store dynamically-generated PCM audio data. See also [`AudioStreamGenerator`][crate::classes::AudioStreamGenerator] for procedural audio generation."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioStreamWav {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AudioStreamWav`][crate::classes::AudioStreamWav].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IAudioStream`][crate::classes::IAudioStream] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AudioStreamWAV` methods](https://docs.godotengine.org/en/stable/classes/class_audiostreamwav.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioStreamWav: crate::obj::GodotClass < Base = AudioStreamWav > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AudioStreamWav {
        #[doc = "Creates a new `AudioStreamWAV` instance from the given buffer. The buffer must contain WAV data.\n\nThe keys and values of `options` match the properties of [`ResourceImporterWAV`][crate::classes::ResourceImporterWav]. The usage of `options` is identical to [`load_from_file`][`crate::classes::AudioStreamWav::load_from_file`]."]
        pub(crate) fn load_from_buffer_full(stream_data: RefArg < PackedByteArray >, options: RefArg < AnyDictionary >,) -> Option < Gd < crate::classes::AudioStreamWav > > {
            type CallRet = Option < Gd < crate::classes::AudioStreamWav > >;
            type CallParams < 'a0, 'a1, > = (RefArg < 'a0, PackedByteArray >, RefArg < 'a1, AnyDictionary >,);
            let args = (stream_data, options,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4571usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamWav", "load_from_buffer", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`load_from_buffer_ex`][Self::load_from_buffer_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a new `AudioStreamWAV` instance from the given buffer. The buffer must contain WAV data.\n\nThe keys and values of `options` match the properties of [`ResourceImporterWAV`][crate::classes::ResourceImporterWav]. The usage of `options` is identical to [`load_from_file`][`crate::classes::AudioStreamWav::load_from_file`]."]
        #[inline]
        pub fn load_from_buffer(stream_data: &PackedByteArray,) -> Option < Gd < crate::classes::AudioStreamWav > > {
            Self::load_from_buffer_ex(stream_data,) . done()
        }
        #[doc = "Creates a new `AudioStreamWAV` instance from the given buffer. The buffer must contain WAV data.\n\nThe keys and values of `options` match the properties of [`ResourceImporterWAV`][crate::classes::ResourceImporterWav]. The usage of `options` is identical to [`load_from_file`][`crate::classes::AudioStreamWav::load_from_file`]."]
        #[inline]
        pub fn load_from_buffer_ex < 'ex > (stream_data: &'ex PackedByteArray,) -> ExLoadFromBuffer < 'ex > {
            ExLoadFromBuffer::new(stream_data,)
        }
        #[doc = "Creates a new `AudioStreamWAV` instance from the given file path. The file must be in WAV format.\n\nThe keys and values of `options` match the properties of [`ResourceImporterWAV`][crate::classes::ResourceImporterWav].\n\n**Example:** Load the first file dropped as a WAV and play it:\n\n```gdscript\n@onready var audio_player = $AudioStreamPlayer\n\nfunc _ready():\n\tget_window().files_dropped.connect(_on_files_dropped)\n\nfunc _on_files_dropped(files):\n\tif files[0].get_extension() == \"wav\":\n\t\taudio_player.stream = AudioStreamWAV.load_from_file(files[0], {\n\t\t\t\t\"force/max_rate\": true,\n\t\t\t\t\"force/max_rate_hz\": 11025\n\t\t\t})\n\t\taudio_player.play()\n```"]
        pub(crate) fn load_from_file_full(path: CowArg < GString >, options: RefArg < AnyDictionary >,) -> Option < Gd < crate::classes::AudioStreamWav > > {
            type CallRet = Option < Gd < crate::classes::AudioStreamWav > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, AnyDictionary >,);
            let args = (path, options,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4572usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamWav", "load_from_file", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`load_from_file_ex`][Self::load_from_file_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a new `AudioStreamWAV` instance from the given file path. The file must be in WAV format.\n\nThe keys and values of `options` match the properties of [`ResourceImporterWAV`][crate::classes::ResourceImporterWav].\n\n**Example:** Load the first file dropped as a WAV and play it:\n\n```gdscript\n@onready var audio_player = $AudioStreamPlayer\n\nfunc _ready():\n\tget_window().files_dropped.connect(_on_files_dropped)\n\nfunc _on_files_dropped(files):\n\tif files[0].get_extension() == \"wav\":\n\t\taudio_player.stream = AudioStreamWAV.load_from_file(files[0], {\n\t\t\t\t\"force/max_rate\": true,\n\t\t\t\t\"force/max_rate_hz\": 11025\n\t\t\t})\n\t\taudio_player.play()\n```"]
        #[inline]
        pub fn load_from_file(path: impl AsArg < GString >,) -> Option < Gd < crate::classes::AudioStreamWav > > {
            Self::load_from_file_ex(path,) . done()
        }
        #[doc = "Creates a new `AudioStreamWAV` instance from the given file path. The file must be in WAV format.\n\nThe keys and values of `options` match the properties of [`ResourceImporterWAV`][crate::classes::ResourceImporterWav].\n\n**Example:** Load the first file dropped as a WAV and play it:\n\n```gdscript\n@onready var audio_player = $AudioStreamPlayer\n\nfunc _ready():\n\tget_window().files_dropped.connect(_on_files_dropped)\n\nfunc _on_files_dropped(files):\n\tif files[0].get_extension() == \"wav\":\n\t\taudio_player.stream = AudioStreamWAV.load_from_file(files[0], {\n\t\t\t\t\"force/max_rate\": true,\n\t\t\t\t\"force/max_rate_hz\": 11025\n\t\t\t})\n\t\taudio_player.play()\n```"]
        #[inline]
        pub fn load_from_file_ex < 'ex > (path: impl AsArg < GString > + 'ex,) -> ExLoadFromFile < 'ex > {
            ExLoadFromFile::new(path,)
        }
        pub fn set_data(&mut self, data: &PackedByteArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >,);
            let args = (RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4573usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamWav", "set_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_data(&self,) -> PackedByteArray {
            type CallRet = PackedByteArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4574usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamWav", "get_data", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_format(&mut self, format: crate::classes::audio_stream_wav::Format,) {
            type CallRet = ();
            type CallParams = (crate::classes::audio_stream_wav::Format,);
            let args = (format,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4575usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamWav", "set_format", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_format(&self,) -> crate::classes::audio_stream_wav::Format {
            type CallRet = crate::classes::audio_stream_wav::Format;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4576usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamWav", "get_format", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_loop_mode(&mut self, loop_mode: crate::classes::audio_stream_wav::LoopMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::audio_stream_wav::LoopMode,);
            let args = (loop_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4577usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamWav", "set_loop_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_loop_mode(&self,) -> crate::classes::audio_stream_wav::LoopMode {
            type CallRet = crate::classes::audio_stream_wav::LoopMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4578usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamWav", "get_loop_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_loop_begin(&mut self, loop_begin: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (loop_begin,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4579usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamWav", "set_loop_begin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_loop_begin(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4580usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamWav", "get_loop_begin", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_loop_end(&mut self, loop_end: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (loop_end,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4581usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamWav", "set_loop_end", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_loop_end(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4582usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamWav", "get_loop_end", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_mix_rate(&mut self, mix_rate: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (mix_rate,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4583usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamWav", "set_mix_rate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mix_rate(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4584usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamWav", "get_mix_rate", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_stereo(&mut self, stereo: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (stereo,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4585usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamWav", "set_stereo", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_stereo(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4586usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamWav", "is_stereo", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_tags(&mut self, tags: &AnyDictionary,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >,);
            let args = (RefArg::new(tags),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4587usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamWav", "set_tags", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_tags(&self,) -> VarDictionary {
            type CallRet = VarDictionary;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4588usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamWav", "get_tags", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Saves the AudioStreamWAV as a WAV file to `path`. Samples with IMA ADPCM or Quite OK Audio formats can't be saved.\n\n**Note:** A `.wav` extension is automatically appended to `path` if it is missing."]
        pub fn save_to_wav(&mut self, path: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(4589usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioStreamWav", "save_to_wav", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AudioStreamWav {
        type Base = crate::classes::AudioStream;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AudioStreamWAV"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioStreamWav {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AudioStream > for AudioStreamWav {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AudioStreamWav {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AudioStreamWav {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioStreamWav {
        
    }
    impl crate::obj::cap::GodotDefault for AudioStreamWav {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioStreamWav {
        type Target = crate::classes::AudioStream;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioStreamWav {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AudioStreamWav`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AudioStreamWav__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioStreamWav > for $Class {
                
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
#[doc = "Default-param extender for [`AudioStreamWav::load_from_buffer_ex`][super::AudioStreamWav::load_from_buffer_ex]."]
#[must_use]
pub struct ExLoadFromBuffer < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, stream_data: CowArg < 'ex, PackedByteArray >, options: CowArg < 'ex, AnyDictionary >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExLoadFromBuffer < 'ex > {
    fn new(stream_data: &'ex PackedByteArray,) -> Self {
        let options = AnyDictionary::new_untyped();
        Self {
            _phantom: std::marker::PhantomData, stream_data: CowArg::Borrowed(stream_data), options: CowArg::Owned(options),
        }
    }
    #[inline]
    pub fn options(self, options: &'ex AnyDictionary) -> Self {
        Self {
            options: CowArg::Borrowed(options), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::AudioStreamWav > > {
        let Self {
            _phantom, stream_data, options,
        }
        = self;
        re_export::AudioStreamWav::load_from_buffer_full(stream_data.cow_as_arg(), options.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`AudioStreamWav::load_from_file_ex`][super::AudioStreamWav::load_from_file_ex]."]
#[must_use]
pub struct ExLoadFromFile < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, path: CowArg < 'ex, GString >, options: CowArg < 'ex, AnyDictionary >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExLoadFromFile < 'ex > {
    fn new(path: impl AsArg < GString > + 'ex,) -> Self {
        let options = AnyDictionary::new_untyped();
        Self {
            _phantom: std::marker::PhantomData, path: path.into_arg(), options: CowArg::Owned(options),
        }
    }
    #[inline]
    pub fn options(self, options: &'ex AnyDictionary) -> Self {
        Self {
            options: CowArg::Borrowed(options), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::AudioStreamWav > > {
        let Self {
            _phantom, path, options,
        }
        = self;
        re_export::AudioStreamWav::load_from_file_full(path, options.cow_as_arg(),)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Format {
    ord: i32
}
impl Format {
    pub const FORMAT_8_BITS: Format = Format {
        ord: 0i32
    };
    pub const FORMAT_16_BITS: Format = Format {
        ord: 1i32
    };
    #[doc(alias = "FORMAT_IMA_ADPCM")]
    #[doc = "Godot enumerator name: `FORMAT_IMA_ADPCM`"]
    pub const IMA_ADPCM: Format = Format {
        ord: 2i32
    };
    #[doc(alias = "FORMAT_QOA")]
    #[doc = "Godot enumerator name: `FORMAT_QOA`"]
    pub const QOA: Format = Format {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Format") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Format {
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
            Self::FORMAT_8_BITS => "FORMAT_8_BITS", Self::FORMAT_16_BITS => "FORMAT_16_BITS", Self::IMA_ADPCM => "IMA_ADPCM", Self::QOA => "QOA", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Format::FORMAT_8_BITS, Format::FORMAT_16_BITS, Format::IMA_ADPCM, Format::QOA]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Format >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("FORMAT_8_BITS", "FORMAT_8_BITS", Format::FORMAT_8_BITS), crate::meta::inspect::EnumConstant::new("FORMAT_16_BITS", "FORMAT_16_BITS", Format::FORMAT_16_BITS), crate::meta::inspect::EnumConstant::new("IMA_ADPCM", "FORMAT_IMA_ADPCM", Format::IMA_ADPCM), crate::meta::inspect::EnumConstant::new("QOA", "FORMAT_QOA", Format::QOA)]
        }
    }
}
impl crate::meta::GodotConvert for Format {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Format 8 Bits", 0i64), EnumeratorShape::new_int("Format 16 Bits", 1i64), EnumeratorShape::new_int("Format Ima Adpcm", 2i64), EnumeratorShape::new_int("Format Qoa", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("AudioStreamWAV.Format")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Format {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Format {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Format {
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
impl crate::registry::property::Export for Format {
    
}
impl crate::meta::Element for Format {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LoopMode {
    ord: i32
}
impl LoopMode {
    #[doc(alias = "LOOP_DISABLED")]
    #[doc = "Godot enumerator name: `LOOP_DISABLED`"]
    pub const DISABLED: LoopMode = LoopMode {
        ord: 0i32
    };
    #[doc(alias = "LOOP_FORWARD")]
    #[doc = "Godot enumerator name: `LOOP_FORWARD`"]
    pub const FORWARD: LoopMode = LoopMode {
        ord: 1i32
    };
    #[doc(alias = "LOOP_PINGPONG")]
    #[doc = "Godot enumerator name: `LOOP_PINGPONG`"]
    pub const PINGPONG: LoopMode = LoopMode {
        ord: 2i32
    };
    #[doc(alias = "LOOP_BACKWARD")]
    #[doc = "Godot enumerator name: `LOOP_BACKWARD`"]
    pub const BACKWARD: LoopMode = LoopMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for LoopMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("LoopMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for LoopMode {
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
            Self::DISABLED => "DISABLED", Self::FORWARD => "FORWARD", Self::PINGPONG => "PINGPONG", Self::BACKWARD => "BACKWARD", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[LoopMode::DISABLED, LoopMode::FORWARD, LoopMode::PINGPONG, LoopMode::BACKWARD]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < LoopMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "LOOP_DISABLED", LoopMode::DISABLED), crate::meta::inspect::EnumConstant::new("FORWARD", "LOOP_FORWARD", LoopMode::FORWARD), crate::meta::inspect::EnumConstant::new("PINGPONG", "LOOP_PINGPONG", LoopMode::PINGPONG), crate::meta::inspect::EnumConstant::new("BACKWARD", "LOOP_BACKWARD", LoopMode::BACKWARD)]
        }
    }
}
impl crate::meta::GodotConvert for LoopMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Loop Disabled", 0i64), EnumeratorShape::new_int("Loop Forward", 1i64), EnumeratorShape::new_int("Loop Pingpong", 2i64), EnumeratorShape::new_int("Loop Backward", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("AudioStreamWAV.LoopMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for LoopMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for LoopMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for LoopMode {
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
impl crate::registry::property::Export for LoopMode {
    
}
impl crate::meta::Element for LoopMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AudioStreamWav;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::audio_stream::SignalsOfAudioStream;
    impl WithSignals for AudioStreamWav {
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
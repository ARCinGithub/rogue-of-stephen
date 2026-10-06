#![doc = "Sidecar module for class [`AudioServer`][crate::classes::AudioServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioServer` enums](https://docs.godotengine.org/en/stable/classes/class_audioserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioServer`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`audio_server`][crate::classes::audio_server]: sidecar module with related enum/flag types\n* [`SignalsOfAudioServer`][crate::classes::audio_server::SignalsOfAudioServer]: signal collection\n\n\nSee also [Godot docs for `AudioServer`](https://docs.godotengine.org/en/stable/classes/class_audioserver.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\n`AudioServer` is a low-level server interface for audio access. It is in charge of creating sample data (playable audio) as well as its playback via a voice interface."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl AudioServer {
        pub fn set_bus_count(&mut self, amount: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (amount,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1285usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "set_bus_count", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_bus_count(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1286usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the bus at index `index`."]
        pub fn remove_bus(&mut self, index: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1287usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "remove_bus", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds a bus at `at_position`."]
        pub(crate) fn add_bus_full(&mut self, at_position: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (at_position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1288usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "add_bus", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_bus_ex`][Self::add_bus_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds a bus at `at_position`."]
        #[inline]
        pub fn add_bus(&mut self,) {
            self.add_bus_ex() . done()
        }
        #[doc = "Adds a bus at `at_position`."]
        #[inline]
        pub fn add_bus_ex < 'ex > (&'ex mut self,) -> ExAddBus < 'ex > {
            ExAddBus::new(self,)
        }
        #[doc = "Moves the bus from index `index` to index `to_index`."]
        pub fn move_bus(&mut self, index: i32, to_index: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (index, to_index,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1289usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "move_bus", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the name of the bus at index `bus_idx` to `name`."]
        pub fn set_bus_name(&mut self, bus_idx: i32, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (bus_idx, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1290usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "set_bus_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the bus with the index `bus_idx`."]
        pub fn get_bus_name(&self, bus_idx: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1291usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the index of the bus with the name `bus_name`. Returns `-1` if no bus with the specified name exist."]
        pub fn get_bus_index(&self, bus_name: impl AsArg < StringName >,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (bus_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1292usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_index", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of channels of the bus at index `bus_idx`."]
        pub fn get_bus_channels(&self, bus_idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1293usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_channels", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the volume in decibels of the bus at index `bus_idx` to `volume_db`."]
        pub fn set_bus_volume_db(&mut self, bus_idx: i32, volume_db: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (bus_idx, volume_db,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1294usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "set_bus_volume_db", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the volume of the bus at index `bus_idx` in dB."]
        pub fn get_bus_volume_db(&self, bus_idx: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1295usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_volume_db", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the volume as a linear value of the bus at index `bus_idx` to `volume_linear`.\n\n**Note:** Using this method is equivalent to calling [`set_bus_volume_db`][`crate::classes::AudioServer::set_bus_volume_db`] with the result of [`linear_to_db`][`crate::global::linear_to_db`] on a value."]
        pub fn set_bus_volume_linear(&mut self, bus_idx: i32, volume_linear: f32,) {
            type CallRet = ();
            type CallParams = (i32, f32,);
            let args = (bus_idx, volume_linear,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1296usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "set_bus_volume_linear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the volume of the bus at index `bus_idx` as a linear value.\n\n**Note:** The returned value is equivalent to the result of [`db_to_linear`][`crate::global::db_to_linear`] on the result of [`get_bus_volume_db`][`crate::classes::AudioServer::get_bus_volume_db`]."]
        pub fn get_bus_volume_linear(&self, bus_idx: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32,);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1297usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_volume_linear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Connects the output of the bus at `bus_idx` to the bus named `send`."]
        pub fn set_bus_send(&mut self, bus_idx: i32, send: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, StringName >,);
            let args = (bus_idx, send.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1298usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "set_bus_send", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the bus that the bus at index `bus_idx` sends to."]
        pub fn get_bus_send(&self, bus_idx: i32,) -> StringName {
            type CallRet = StringName;
            type CallParams = (i32,);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1299usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_send", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the bus at index `bus_idx` is in solo mode."]
        pub fn set_bus_solo(&mut self, bus_idx: i32, enable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (bus_idx, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1300usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "set_bus_solo", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the bus at index `bus_idx` is in solo mode."]
        pub fn is_bus_solo(&self, bus_idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1301usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "is_bus_solo", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the bus at index `bus_idx` is muted."]
        pub fn set_bus_mute(&mut self, bus_idx: i32, enable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (bus_idx, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1302usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "set_bus_mute", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the bus at index `bus_idx` is muted."]
        pub fn is_bus_mute(&self, bus_idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1303usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "is_bus_mute", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the bus at index `bus_idx` is bypassing effects."]
        pub fn set_bus_bypass_effects(&mut self, bus_idx: i32, enable: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (bus_idx, enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1304usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "set_bus_bypass_effects", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the bus at index `bus_idx` is bypassing effects."]
        pub fn is_bus_bypassing_effects(&self, bus_idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32,);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1305usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "is_bus_bypassing_effects", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Adds an [`AudioEffect`][crate::classes::AudioEffect] effect to the bus `bus_idx` at `at_position`."]
        pub(crate) fn add_bus_effect_full(&mut self, bus_idx: i32, effect: CowArg < Option < Gd < crate::classes::AudioEffect > > >, at_position: i32,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::AudioEffect > > >, i32,);
            let args = (bus_idx, effect, at_position,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1306usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "add_bus_effect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`add_bus_effect_ex`][Self::add_bus_effect_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Adds an [`AudioEffect`][crate::classes::AudioEffect] effect to the bus `bus_idx` at `at_position`."]
        #[inline]
        pub fn add_bus_effect(&mut self, bus_idx: i32, effect: impl AsArg < Option < Gd < crate::classes::AudioEffect >> >,) {
            self.add_bus_effect_ex(bus_idx, effect,) . done()
        }
        #[doc = "Adds an [`AudioEffect`][crate::classes::AudioEffect] effect to the bus `bus_idx` at `at_position`."]
        #[inline]
        pub fn add_bus_effect_ex < 'ex > (&'ex mut self, bus_idx: i32, effect: impl AsArg < Option < Gd < crate::classes::AudioEffect >> > + 'ex,) -> ExAddBusEffect < 'ex > {
            ExAddBusEffect::new(self, bus_idx, effect,)
        }
        #[doc = "Removes the effect at index `effect_idx` from the bus at index `bus_idx`."]
        pub fn remove_bus_effect(&mut self, bus_idx: i32, effect_idx: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32,);
            let args = (bus_idx, effect_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1307usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "remove_bus_effect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of effects on the bus at `bus_idx`."]
        pub fn get_bus_effect_count(&self, bus_idx: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (bus_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1308usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_effect_count", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`AudioEffect`][crate::classes::AudioEffect] at position `effect_idx` in bus `bus_idx`."]
        pub fn get_bus_effect(&self, bus_idx: i32, effect_idx: i32,) -> Option < Gd < crate::classes::AudioEffect > > {
            type CallRet = Option < Gd < crate::classes::AudioEffect > >;
            type CallParams = (i32, i32,);
            let args = (bus_idx, effect_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1309usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_effect", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`AudioEffectInstance`][crate::classes::AudioEffectInstance] assigned to the given bus and effect indices (and optionally channel)."]
        pub(crate) fn get_bus_effect_instance_full(&self, bus_idx: i32, effect_idx: i32, channel: i32,) -> Option < Gd < crate::classes::AudioEffectInstance > > {
            type CallRet = Option < Gd < crate::classes::AudioEffectInstance > >;
            type CallParams = (i32, i32, i32,);
            let args = (bus_idx, effect_idx, channel,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1310usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_effect_instance", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`get_bus_effect_instance_ex`][Self::get_bus_effect_instance_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns the [`AudioEffectInstance`][crate::classes::AudioEffectInstance] assigned to the given bus and effect indices (and optionally channel)."]
        #[inline]
        pub fn get_bus_effect_instance(&self, bus_idx: i32, effect_idx: i32,) -> Option < Gd < crate::classes::AudioEffectInstance > > {
            self.get_bus_effect_instance_ex(bus_idx, effect_idx,) . done()
        }
        #[doc = "Returns the [`AudioEffectInstance`][crate::classes::AudioEffectInstance] assigned to the given bus and effect indices (and optionally channel)."]
        #[inline]
        pub fn get_bus_effect_instance_ex < 'ex > (&'ex self, bus_idx: i32, effect_idx: i32,) -> ExGetBusEffectInstance < 'ex > {
            ExGetBusEffectInstance::new(self, bus_idx, effect_idx,)
        }
        #[doc = "Swaps the position of two effects in bus `bus_idx`."]
        pub fn swap_bus_effects(&mut self, bus_idx: i32, effect_idx: i32, by_effect_idx: i32,) {
            type CallRet = ();
            type CallParams = (i32, i32, i32,);
            let args = (bus_idx, effect_idx, by_effect_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1311usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "swap_bus_effects", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the effect at index `effect_idx` on the bus at index `bus_idx` is enabled."]
        pub fn set_bus_effect_enabled(&mut self, bus_idx: i32, effect_idx: i32, enabled: bool,) {
            type CallRet = ();
            type CallParams = (i32, i32, bool,);
            let args = (bus_idx, effect_idx, enabled,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1312usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "set_bus_effect_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the effect at index `effect_idx` on the bus at index `bus_idx` is enabled."]
        pub fn is_bus_effect_enabled(&self, bus_idx: i32, effect_idx: i32,) -> bool {
            type CallRet = bool;
            type CallParams = (i32, i32,);
            let args = (bus_idx, effect_idx,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1313usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "is_bus_effect_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the peak volume of the left speaker at bus index `bus_idx` and channel index `channel`."]
        pub fn get_bus_peak_volume_left_db(&self, bus_idx: i32, channel: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (bus_idx, channel,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1314usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_peak_volume_left_db", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the peak volume of the right speaker at bus index `bus_idx` and channel index `channel`."]
        pub fn get_bus_peak_volume_right_db(&self, bus_idx: i32, channel: i32,) -> f32 {
            type CallRet = f32;
            type CallParams = (i32, i32,);
            let args = (bus_idx, channel,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1315usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_bus_peak_volume_right_db", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_playback_speed_scale(&mut self, scale: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (scale,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1316usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "set_playback_speed_scale", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_playback_speed_scale(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1317usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_playback_speed_scale", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Locks the audio driver's main loop.\n\n**Note:** Remember to unlock it afterwards."]
        pub fn lock(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1318usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "lock", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Unlocks the audio driver's main loop. (After locking it, you should always unlock it.)"]
        pub fn unlock(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1319usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "unlock", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the speaker configuration."]
        pub fn get_speaker_mode(&self,) -> crate::classes::audio_server::SpeakerMode {
            type CallRet = crate::classes::audio_server::SpeakerMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1320usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_speaker_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the sample rate at the output of the `AudioServer`."]
        pub fn get_mix_rate(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1321usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_mix_rate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the sample rate at the input of the `AudioServer`."]
        pub fn get_input_mix_rate(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1322usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_input_mix_rate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the name of the current audio driver. The default usually depends on the operating system, but may be overridden via the `--audio-driver` [command line argument]($DOCS_URL/tutorials/editor/command_line_tutorial.html). `--headless` also automatically sets the audio driver to `Dummy`. See also \\[member ProjectSettings.audio/driver/driver]."]
        pub fn get_driver_name(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1323usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_driver_name", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the names of all audio output devices detected on the system."]
        pub fn get_output_device_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1324usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_output_device_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_output_device(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1325usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_output_device", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_output_device(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1326usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "set_output_device", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the relative time until the next mix occurs."]
        pub fn get_time_to_next_mix(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1327usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_time_to_next_mix", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the relative time since the last mix occurred."]
        pub fn get_time_since_last_mix(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1328usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_time_since_last_mix", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the audio driver's effective output latency. This is based on \\[member ProjectSettings.audio/driver/output_latency], but the exact returned value will differ depending on the operating system and audio driver.\n\n**Note:** This can be expensive; it is not recommended to call [`get_output_latency`][`crate::classes::AudioServer::get_output_latency`] every frame."]
        pub fn get_output_latency(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1329usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_output_latency", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the names of all audio input devices detected on the system.\n\n**Note:** \\[member ProjectSettings.audio/driver/enable_input] must be `true` for audio input to work. See also that setting's description for caveats related to permissions and operating system privacy settings."]
        pub fn get_input_device_list(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1330usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_input_device_list", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_input_device(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1331usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_input_device", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_input_device(&mut self, name: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (name.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1332usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "set_input_device", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `active` is `true`, starts the microphone input stream specified by \\[member input_device] or returns an error if it failed.\n\nIf `active` is `false`, stops the input stream if it is running."]
        pub fn set_input_device_active(&mut self, active: bool,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (bool,);
            let args = (active,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1333usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "set_input_device_active", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of frames available to read using [`get_input_frames`][`crate::classes::AudioServer::get_input_frames`]."]
        pub fn get_input_frames_available(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1334usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_input_frames_available", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the absolute size of the microphone input buffer. This is set to a multiple of the audio latency and can be used to estimate the minimum rate at which the frames need to be fetched."]
        pub fn get_input_buffer_length_frames(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1335usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_input_buffer_length_frames", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a [`PackedVector2Array`][crate::builtin::PackedVector2Array] containing exactly `frames` audio samples from the internal microphone buffer if available, otherwise returns an empty [`PackedVector2Array`][crate::builtin::PackedVector2Array].\n\nThe buffer is filled at the rate of [`get_input_mix_rate`][`crate::classes::AudioServer::get_input_mix_rate`] frames per second when [`set_input_device_active`][`crate::classes::AudioServer::set_input_device_active`] has successfully been set to `true`.\n\nThe samples are signed floating-point PCM values between `-1` and `1`."]
        pub fn get_input_frames(&self, frames: i32,) -> PackedVector2Array {
            type CallRet = PackedVector2Array;
            type CallParams = (i32,);
            let args = (frames,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1336usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "get_input_frames", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Overwrites the currently used [`AudioBusLayout`][crate::classes::AudioBusLayout]."]
        pub fn set_bus_layout(&mut self, bus_layout: impl AsArg < Option < Gd < crate::classes::AudioBusLayout >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::AudioBusLayout > > >,);
            let args = (bus_layout.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1337usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "set_bus_layout", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Generates an [`AudioBusLayout`][crate::classes::AudioBusLayout] using the available buses and effects."]
        pub fn generate_bus_layout(&self,) -> Option < Gd < crate::classes::AudioBusLayout > > {
            type CallRet = Option < Gd < crate::classes::AudioBusLayout > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1338usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "generate_bus_layout", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If set to `true`, all instances of [`AudioStreamPlayback`][crate::classes::AudioStreamPlayback] will call [`tag_used_streams`][`crate::classes::IAudioStreamPlayback::tag_used_streams`] every mix step.\n\n**Note:** This is enabled by default in the editor, as it is used by editor plugins for the audio stream previews."]
        pub fn set_enable_tagging_used_audio_streams(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1339usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "set_enable_tagging_used_audio_streams", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `true`, the stream is registered as a sample. The engine will not have to register it before playing the sample.\n\nIf `false`, the stream will have to be registered before playing it. To prevent lag spikes, register the stream as sample with [`register_stream_as_sample`][`crate::classes::AudioServer::register_stream_as_sample`]."]
        pub fn is_stream_registered_as_sample(&self, stream: impl AsArg < Option < Gd < crate::classes::AudioStream >> >,) -> bool {
            type CallRet = bool;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::AudioStream > > >,);
            let args = (stream.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1340usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "is_stream_registered_as_sample", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Forces the registration of a stream as a sample.\n\n**Note:** Lag spikes may occur when calling this method, especially on single-threaded builds. It is suggested to call this method while loading assets, where the lag spike could be masked, instead of registering the sample right before it needs to be played."]
        pub fn register_stream_as_sample(&mut self, stream: impl AsArg < Option < Gd < crate::classes::AudioStream >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::AudioStream > > >,);
            let args = (stream.into_arg(),);
            unsafe {
                let method_bind = sys::class_servers_api() . fptr_by_index(1341usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioServer", "register_stream_as_sample", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AudioServer {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AudioServer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Servers;
        
    }
    unsafe impl crate::obj::Bounds for AudioServer {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioServer {
        
    }
    impl crate::obj::Singleton for AudioServer {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"AudioServer"))
            }
        }
    }
    impl std::ops::Deref for AudioServer {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AudioServer__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `AudioServer` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`AudioServer::add_bus_ex`][super::AudioServer::add_bus_ex]."]
#[must_use]
pub struct ExAddBus < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AudioServer, at_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddBus < 'ex > {
    fn new(surround_object: &'ex mut re_export::AudioServer,) -> Self {
        let at_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, at_position: at_position,
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
            _phantom, surround_object, at_position,
        }
        = self;
        re_export::AudioServer::add_bus_full(surround_object, at_position,)
    }
}
#[doc = "Default-param extender for [`AudioServer::add_bus_effect_ex`][super::AudioServer::add_bus_effect_ex]."]
#[must_use]
pub struct ExAddBusEffect < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::AudioServer, bus_idx: i32, effect: CowArg < 'ex, Option < Gd < crate::classes::AudioEffect > > >, at_position: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExAddBusEffect < 'ex > {
    fn new(surround_object: &'ex mut re_export::AudioServer, bus_idx: i32, effect: impl AsArg < Option < Gd < crate::classes::AudioEffect >> > + 'ex,) -> Self {
        let at_position = - 1i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, bus_idx: bus_idx, effect: effect.into_arg(), at_position: at_position,
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
            _phantom, surround_object, bus_idx, effect, at_position,
        }
        = self;
        re_export::AudioServer::add_bus_effect_full(surround_object, bus_idx, effect, at_position,)
    }
}
#[doc = "Default-param extender for [`AudioServer::get_bus_effect_instance_ex`][super::AudioServer::get_bus_effect_instance_ex]."]
#[must_use]
pub struct ExGetBusEffectInstance < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex re_export::AudioServer, bus_idx: i32, effect_idx: i32, channel: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExGetBusEffectInstance < 'ex > {
    fn new(surround_object: &'ex re_export::AudioServer, bus_idx: i32, effect_idx: i32,) -> Self {
        let channel = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, bus_idx: bus_idx, effect_idx: effect_idx, channel: channel,
        }
    }
    #[inline]
    pub fn channel(self, channel: i32) -> Self {
        Self {
            channel: channel, .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::AudioEffectInstance > > {
        let Self {
            _phantom, surround_object, bus_idx, effect_idx, channel,
        }
        = self;
        re_export::AudioServer::get_bus_effect_instance_full(surround_object, bus_idx, effect_idx, channel,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SpeakerMode {
    ord: i32
}
impl SpeakerMode {
    #[doc(alias = "SPEAKER_MODE_STEREO")]
    #[doc = "Godot enumerator name: `SPEAKER_MODE_STEREO`"]
    pub const STEREO: SpeakerMode = SpeakerMode {
        ord: 0i32
    };
    #[doc(alias = "SPEAKER_SURROUND_31")]
    #[doc = "Godot enumerator name: `SPEAKER_SURROUND_31`"]
    pub const SURROUND_31: SpeakerMode = SpeakerMode {
        ord: 1i32
    };
    #[doc(alias = "SPEAKER_SURROUND_51")]
    #[doc = "Godot enumerator name: `SPEAKER_SURROUND_51`"]
    pub const SURROUND_51: SpeakerMode = SpeakerMode {
        ord: 2i32
    };
    #[doc(alias = "SPEAKER_SURROUND_71")]
    #[doc = "Godot enumerator name: `SPEAKER_SURROUND_71`"]
    pub const SURROUND_71: SpeakerMode = SpeakerMode {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for SpeakerMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SpeakerMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SpeakerMode {
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
            Self::STEREO => "STEREO", Self::SURROUND_31 => "SURROUND_31", Self::SURROUND_51 => "SURROUND_51", Self::SURROUND_71 => "SURROUND_71", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SpeakerMode::STEREO, SpeakerMode::SURROUND_31, SpeakerMode::SURROUND_51, SpeakerMode::SURROUND_71]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SpeakerMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("STEREO", "SPEAKER_MODE_STEREO", SpeakerMode::STEREO), crate::meta::inspect::EnumConstant::new("SURROUND_31", "SPEAKER_SURROUND_31", SpeakerMode::SURROUND_31), crate::meta::inspect::EnumConstant::new("SURROUND_51", "SPEAKER_SURROUND_51", SpeakerMode::SURROUND_51), crate::meta::inspect::EnumConstant::new("SURROUND_71", "SPEAKER_SURROUND_71", SpeakerMode::SURROUND_71)]
        }
    }
}
impl crate::meta::GodotConvert for SpeakerMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Speaker Mode Stereo", 0i64), EnumeratorShape::new_int("Speaker Surround 31", 1i64), EnumeratorShape::new_int("Speaker Surround 51", 2i64), EnumeratorShape::new_int("Speaker Surround 71", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("AudioServer.SpeakerMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for SpeakerMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SpeakerMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for SpeakerMode {
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
impl crate::registry::property::Export for SpeakerMode {
    
}
impl crate::meta::Element for SpeakerMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct PlaybackType {
    ord: i32
}
impl PlaybackType {
    #[doc(alias = "PLAYBACK_TYPE_DEFAULT")]
    #[doc = "Godot enumerator name: `PLAYBACK_TYPE_DEFAULT`"]
    pub const DEFAULT: PlaybackType = PlaybackType {
        ord: 0i32
    };
    #[doc(alias = "PLAYBACK_TYPE_STREAM")]
    #[doc = "Godot enumerator name: `PLAYBACK_TYPE_STREAM`"]
    pub const STREAM: PlaybackType = PlaybackType {
        ord: 1i32
    };
    #[doc(alias = "PLAYBACK_TYPE_SAMPLE")]
    #[doc = "Godot enumerator name: `PLAYBACK_TYPE_SAMPLE`"]
    pub const SAMPLE: PlaybackType = PlaybackType {
        ord: 2i32
    };
    #[doc(alias = "PLAYBACK_TYPE_MAX")]
    #[doc = "Godot enumerator name: `PLAYBACK_TYPE_MAX`"]
    pub const MAX: PlaybackType = PlaybackType {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for PlaybackType {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("PlaybackType") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for PlaybackType {
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
            Self::DEFAULT => "DEFAULT", Self::STREAM => "STREAM", Self::SAMPLE => "SAMPLE", Self::MAX => "MAX", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[PlaybackType::DEFAULT, PlaybackType::STREAM, PlaybackType::SAMPLE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < PlaybackType >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DEFAULT", "PLAYBACK_TYPE_DEFAULT", PlaybackType::DEFAULT), crate::meta::inspect::EnumConstant::new("STREAM", "PLAYBACK_TYPE_STREAM", PlaybackType::STREAM), crate::meta::inspect::EnumConstant::new("SAMPLE", "PLAYBACK_TYPE_SAMPLE", PlaybackType::SAMPLE), crate::meta::inspect::EnumConstant::new("MAX", "PLAYBACK_TYPE_MAX", PlaybackType::MAX)]
        }
    }
}
impl crate::obj::IndexEnum for PlaybackType {
    const ENUMERATOR_COUNT: usize = 3usize;
    
}
impl crate::meta::GodotConvert for PlaybackType {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Playback Type Default", 0i64), EnumeratorShape::new_int("Playback Type Stream", 1i64), EnumeratorShape::new_int("Playback Type Sample", 2i64), EnumeratorShape::new_int("Playback Type Max", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("AudioServer.PlaybackType")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for PlaybackType {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for PlaybackType {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for PlaybackType {
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
impl crate::registry::property::Export for PlaybackType {
    
}
impl crate::meta::Element for PlaybackType {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AudioServer;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`AudioServer`][crate::classes::AudioServer] class."]
    pub struct SignalsOfAudioServer < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfAudioServer < 'c, C > {
        #[doc = "Signature: `()`"]
        pub fn bus_layout_changed(&mut self) -> SigBusLayoutChanged < 'c, C > {
            SigBusLayoutChanged {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "bus_layout_changed")
            }
        }
        #[doc = "Signature: `(bus_index: i64, old_name: StringName, new_name: StringName)`"]
        pub fn bus_renamed(&mut self) -> SigBusRenamed < 'c, C > {
            SigBusRenamed {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "bus_renamed")
            }
        }
    }
    type TypedSigBusLayoutChanged < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigBusLayoutChanged < 'c, C: WithSignals > {
        typed: TypedSigBusLayoutChanged < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigBusLayoutChanged < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigBusLayoutChanged < 'c, C > {
        type Target = TypedSigBusLayoutChanged < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigBusLayoutChanged < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigBusRenamed < 'c, C > = TypedSignal < 'c, C, (i64, StringName, StringName,) >;
    pub struct SigBusRenamed < 'c, C: WithSignals > {
        typed: TypedSigBusRenamed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigBusRenamed < 'c, C > {
        pub fn emit(&mut self, bus_index: i64, old_name: StringName, new_name: StringName,) {
            self.typed.emit_tuple((bus_index, old_name, new_name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigBusRenamed < 'c, C > {
        type Target = TypedSigBusRenamed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigBusRenamed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for AudioServer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfAudioServer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfAudioServer < 'c, C > {
        type Target = < < AudioServer as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = AudioServer;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfAudioServer < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = AudioServer;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
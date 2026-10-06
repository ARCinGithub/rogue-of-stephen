#![doc = "Sidecar module for class [`AudioEffectDistortion`][crate::classes::AudioEffectDistortion].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AudioEffectDistortion` enums](https://docs.godotengine.org/en/stable/classes/class_audioeffectdistortion.html#enumerations).\n\n"]
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
    #[doc = "Godot class `AudioEffectDistortion`.\n\nInherits [`AudioEffect`][crate::classes::AudioEffect].\n\nRelated symbols:\n\n* [`audio_effect_distortion`][crate::classes::audio_effect_distortion]: sidecar module with related enum/flag types\n* [`IAudioEffectDistortion`][crate::classes::IAudioEffectDistortion]: virtual methods\n\n\nSee also [Godot docs for `AudioEffectDistortion`](https://docs.godotengine.org/en/stable/classes/class_audioeffectdistortion.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`AudioEffectDistortion::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nDifferent types are available: clip, tan, lo-fi (bit crushing), overdrive, or waveshape.\n\nBy distorting the waveform the frequency content changes, which will often make the sound \"crunchy\" or \"abrasive\". For games, it can simulate sound coming from some saturated device or speaker very efficiently."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct AudioEffectDistortion {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`AudioEffectDistortion`][crate::classes::AudioEffectDistortion].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IAudioEffect`][crate::classes::IAudioEffect] > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `AudioEffectDistortion` methods](https://docs.godotengine.org/en/stable/classes/class_audioeffectdistortion.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IAudioEffectDistortion: crate::obj::GodotClass < Base = AudioEffectDistortion > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl AudioEffectDistortion {
        pub fn set_mode(&mut self, mode: crate::classes::audio_effect_distortion::Mode,) {
            type CallRet = ();
            type CallParams = (crate::classes::audio_effect_distortion::Mode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6943usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectDistortion", "set_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_mode(&self,) -> crate::classes::audio_effect_distortion::Mode {
            type CallRet = crate::classes::audio_effect_distortion::Mode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6944usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectDistortion", "get_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pre_gain(&mut self, pre_gain: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (pre_gain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6945usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectDistortion", "set_pre_gain", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pre_gain(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6946usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectDistortion", "get_pre_gain", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_keep_hf_hz(&mut self, keep_hf_hz: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (keep_hf_hz,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6947usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectDistortion", "set_keep_hf_hz", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_keep_hf_hz(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6948usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectDistortion", "get_keep_hf_hz", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_drive(&mut self, drive: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (drive,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6949usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectDistortion", "set_drive", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_drive(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6950usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectDistortion", "get_drive", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_post_gain(&mut self, post_gain: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (post_gain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6951usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectDistortion", "set_post_gain", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_post_gain(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(6952usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "AudioEffectDistortion", "get_post_gain", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for AudioEffectDistortion {
        type Base = crate::classes::AudioEffect;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("AudioEffectDistortion"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for AudioEffectDistortion {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::AudioEffect > for AudioEffectDistortion {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for AudioEffectDistortion {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for AudioEffectDistortion {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for AudioEffectDistortion {
        
    }
    impl crate::obj::cap::GodotDefault for AudioEffectDistortion {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for AudioEffectDistortion {
        type Target = crate::classes::AudioEffect;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for AudioEffectDistortion {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`AudioEffectDistortion`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_AudioEffectDistortion__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::AudioEffectDistortion > for $Class {
                
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Mode {
    ord: i32
}
impl Mode {
    #[doc(alias = "MODE_CLIP")]
    #[doc = "Godot enumerator name: `MODE_CLIP`"]
    pub const CLIP: Mode = Mode {
        ord: 0i32
    };
    #[doc(alias = "MODE_ATAN")]
    #[doc = "Godot enumerator name: `MODE_ATAN`"]
    pub const ATAN: Mode = Mode {
        ord: 1i32
    };
    #[doc(alias = "MODE_LOFI")]
    #[doc = "Godot enumerator name: `MODE_LOFI`"]
    pub const LOFI: Mode = Mode {
        ord: 2i32
    };
    #[doc(alias = "MODE_OVERDRIVE")]
    #[doc = "Godot enumerator name: `MODE_OVERDRIVE`"]
    pub const OVERDRIVE: Mode = Mode {
        ord: 3i32
    };
    #[doc(alias = "MODE_WAVESHAPE")]
    #[doc = "Godot enumerator name: `MODE_WAVESHAPE`"]
    pub const WAVESHAPE: Mode = Mode {
        ord: 4i32
    };
    
}
impl std::fmt::Debug for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Mode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Mode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self {
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
            Self::CLIP => "CLIP", Self::ATAN => "ATAN", Self::LOFI => "LOFI", Self::OVERDRIVE => "OVERDRIVE", Self::WAVESHAPE => "WAVESHAPE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Mode::CLIP, Mode::ATAN, Mode::LOFI, Mode::OVERDRIVE, Mode::WAVESHAPE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Mode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CLIP", "MODE_CLIP", Mode::CLIP), crate::meta::inspect::EnumConstant::new("ATAN", "MODE_ATAN", Mode::ATAN), crate::meta::inspect::EnumConstant::new("LOFI", "MODE_LOFI", Mode::LOFI), crate::meta::inspect::EnumConstant::new("OVERDRIVE", "MODE_OVERDRIVE", Mode::OVERDRIVE), crate::meta::inspect::EnumConstant::new("WAVESHAPE", "MODE_WAVESHAPE", Mode::WAVESHAPE)]
        }
    }
}
impl crate::meta::GodotConvert for Mode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Mode Clip", 0i64), EnumeratorShape::new_int("Mode Atan", 1i64), EnumeratorShape::new_int("Mode Lofi", 2i64), EnumeratorShape::new_int("Mode Overdrive", 3i64), EnumeratorShape::new_int("Mode Waveshape", 4i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("AudioEffectDistortion.Mode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Mode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Mode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Mode {
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
impl crate::registry::property::Export for Mode {
    
}
impl crate::meta::Element for Mode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::AudioEffectDistortion;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for AudioEffectDistortion {
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
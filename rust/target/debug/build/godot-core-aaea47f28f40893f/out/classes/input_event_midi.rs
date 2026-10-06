#![doc = "Sidecar module for class [`InputEventMidi`][crate::classes::InputEventMidi].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `InputEventMIDI` enums](https://docs.godotengine.org/en/stable/classes/class_inputeventmidi.html#enumerations).\n\n"]
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
    #[doc = "Godot class `InputEventMIDI`.\n\nInherits [`InputEvent`][crate::classes::InputEvent].\n\nRelated symbols:\n\n* [`IInputEventMidi`][crate::classes::IInputEventMidi]: virtual methods\n\n\nSee also [Godot docs for `InputEventMIDI`](https://docs.godotengine.org/en/stable/classes/class_inputeventmidi.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`InputEventMidi::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nInputEventMIDI stores information about messages from [MIDI](https://en.wikipedia.org/wiki/MIDI) (Musical Instrument Digital Interface) devices. These may include musical keyboards, synthesizers, and drum machines.\n\nMIDI messages can be received over a 5-pin MIDI connector or over USB. If your device supports both be sure to check the settings in the device to see which output it is using.\n\nBy default, Godot does not detect MIDI devices. You need to call [`open_midi_inputs`][`crate::classes::Os::open_midi_inputs`], first. You can check which devices are detected with [`get_connected_midi_inputs`][`crate::classes::Os::get_connected_midi_inputs`], and close the connection with [`close_midi_inputs`][`crate::classes::Os::close_midi_inputs`].\n\n\n```gdscript\nfunc _ready():\n\tOS.open_midi_inputs()\n\tprint(OS.get_connected_midi_inputs())\n\nfunc _input(input_event):\n\tif input_event is InputEventMIDI:\n\t\t_print_midi_info(input_event)\n\nfunc _print_midi_info(midi_event):\n\tprint(midi_event)\n\tprint(\"Channel \", midi_event.channel)\n\tprint(\"Message \", midi_event.message)\n\tprint(\"Pitch \", midi_event.pitch)\n\tprint(\"Velocity \", midi_event.velocity)\n\tprint(\"Instrument \", midi_event.instrument)\n\tprint(\"Pressure \", midi_event.pressure)\n\tprint(\"Controller number: \", midi_event.controller_number)\n\tprint(\"Controller value: \", midi_event.controller_value)\n```\n\n\n**Note:** Godot does not support MIDI output, so there is no way to emit MIDI messages from Godot. Only MIDI input is supported.\n\n**Note:** On the Web platform, using MIDI input requires a browser permission to be granted first. This permission request is performed when calling [`open_midi_inputs`][`crate::classes::Os::open_midi_inputs`]. MIDI input will not work until the user accepts the permission request."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct InputEventMidi {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`InputEventMidi`][crate::classes::InputEventMidi].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IInputEvent`~~ > [`IResource`][crate::classes::IResource] > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `InputEventMIDI` methods](https://docs.godotengine.org/en/stable/classes/class_inputeventmidi.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IInputEventMidi: crate::obj::GodotClass < Base = InputEventMidi > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl InputEventMidi {
        pub fn set_channel(&mut self, channel: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (channel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11392usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventMidi", "set_channel", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_channel(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11393usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventMidi", "get_channel", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_message(&mut self, message: crate::global::MidiMessage,) {
            type CallRet = ();
            type CallParams = (crate::global::MidiMessage,);
            let args = (message,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11394usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventMidi", "set_message", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_message(&self,) -> crate::global::MidiMessage {
            type CallRet = crate::global::MidiMessage;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11395usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventMidi", "get_message", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pitch(&mut self, pitch: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (pitch,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11396usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventMidi", "set_pitch", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pitch(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11397usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventMidi", "get_pitch", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_velocity(&mut self, velocity: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (velocity,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11398usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventMidi", "set_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_velocity(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11399usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventMidi", "get_velocity", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_instrument(&mut self, instrument: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (instrument,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11400usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventMidi", "set_instrument", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_instrument(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11401usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventMidi", "get_instrument", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_pressure(&mut self, pressure: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (pressure,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11402usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventMidi", "set_pressure", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_pressure(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11403usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventMidi", "get_pressure", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_controller_number(&mut self, controller_number: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (controller_number,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11404usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventMidi", "set_controller_number", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_controller_number(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11405usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventMidi", "get_controller_number", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_controller_value(&mut self, controller_value: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (controller_value,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11406usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventMidi", "set_controller_value", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_controller_value(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11407usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "InputEventMidi", "get_controller_value", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for InputEventMidi {
        type Base = crate::classes::InputEvent;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("InputEventMIDI"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for InputEventMidi {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::Yes;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::InputEvent > for InputEventMidi {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Resource > for InputEventMidi {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for InputEventMidi {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for InputEventMidi {
        
    }
    impl crate::obj::cap::GodotDefault for InputEventMidi {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for InputEventMidi {
        type Target = crate::classes::InputEvent;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for InputEventMidi {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`InputEventMidi`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_InputEventMidi__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::InputEventMidi > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::InputEvent > for $Class {
                
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
    use super::re_export::InputEventMidi;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::resource::SignalsOfResource;
    impl WithSignals for InputEventMidi {
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
#![doc = "Sidecar module for class [`WebRtcDataChannel`][crate::classes::WebRtcDataChannel].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `WebRTCDataChannel` enums](https://docs.godotengine.org/en/stable/classes/class_webrtcdatachannel.html#enumerations).\n\n"]
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
    #[doc = "Godot class `WebRTCDataChannel`.\n\nInherits [`PacketPeer`][crate::classes::PacketPeer].\n\nRelated symbols:\n\n* [`web_rtc_data_channel`][crate::classes::web_rtc_data_channel]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `WebRTCDataChannel`](https://docs.godotengine.org/en/stable/classes/class_webrtcdatachannel.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<WebRtcDataChannel>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct WebRtcDataChannel {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl WebRtcDataChannel {
        #[doc = "Reserved, but not used for now."]
        pub fn poll(&mut self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5638usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "poll", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Closes this data channel, notifying the other peer."]
        pub fn close(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5639usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "close", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the last received packet was transferred as text. See \\[member write_mode]."]
        pub fn was_string_packet(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5640usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "was_string_packet", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_write_mode(&mut self, write_mode: crate::classes::web_rtc_data_channel::WriteMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::web_rtc_data_channel::WriteMode,);
            let args = (write_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5641usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "set_write_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_write_mode(&self,) -> crate::classes::web_rtc_data_channel::WriteMode {
            type CallRet = crate::classes::web_rtc_data_channel::WriteMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5642usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "get_write_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current state of this channel."]
        pub fn get_ready_state(&self,) -> crate::classes::web_rtc_data_channel::ChannelState {
            type CallRet = crate::classes::web_rtc_data_channel::ChannelState;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5643usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "get_ready_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the label assigned to this channel during creation."]
        pub fn get_label(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5644usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "get_label", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if this channel was created with ordering enabled (default)."]
        pub fn is_ordered(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5645usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "is_ordered", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the ID assigned to this channel during creation (or auto-assigned during negotiation).\n\nIf the channel is not negotiated out-of-band the ID will only be available after the connection is established (will return `65535` until then)."]
        pub fn get_id(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5646usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "get_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the `maxPacketLifeTime` value assigned to this channel during creation.\n\nWill be `65535` if not specified."]
        pub fn get_max_packet_life_time(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5647usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "get_max_packet_life_time", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the `maxRetransmits` value assigned to this channel during creation.\n\nWill be `65535` if not specified."]
        pub fn get_max_retransmits(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5648usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "get_max_retransmits", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the sub-protocol assigned to this channel during creation. An empty string if not specified."]
        pub fn get_protocol(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5649usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "get_protocol", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if this channel was created with out-of-band configuration."]
        pub fn is_negotiated(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5650usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "is_negotiated", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the number of bytes currently queued to be sent over this channel."]
        pub fn get_buffered_amount(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(5651usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcDataChannel", "get_buffered_amount", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for WebRtcDataChannel {
        type Base = crate::classes::PacketPeer;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("WebRTCDataChannel"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for WebRtcDataChannel {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PacketPeer > for WebRtcDataChannel {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for WebRtcDataChannel {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for WebRtcDataChannel {
        
    }
    impl std::ops::Deref for WebRtcDataChannel {
        type Target = crate::classes::PacketPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for WebRtcDataChannel {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_WebRtcDataChannel__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `WebRtcDataChannel` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct WriteMode {
    ord: i32
}
impl WriteMode {
    #[doc(alias = "WRITE_MODE_TEXT")]
    #[doc = "Godot enumerator name: `WRITE_MODE_TEXT`"]
    pub const TEXT: WriteMode = WriteMode {
        ord: 0i32
    };
    #[doc(alias = "WRITE_MODE_BINARY")]
    #[doc = "Godot enumerator name: `WRITE_MODE_BINARY`"]
    pub const BINARY: WriteMode = WriteMode {
        ord: 1i32
    };
    
}
impl std::fmt::Debug for WriteMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("WriteMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for WriteMode {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self {
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
            Self::TEXT => "TEXT", Self::BINARY => "BINARY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[WriteMode::TEXT, WriteMode::BINARY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < WriteMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("TEXT", "WRITE_MODE_TEXT", WriteMode::TEXT), crate::meta::inspect::EnumConstant::new("BINARY", "WRITE_MODE_BINARY", WriteMode::BINARY)]
        }
    }
}
impl crate::meta::GodotConvert for WriteMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Write Mode Text", 0i64), EnumeratorShape::new_int("Write Mode Binary", 1i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("WebRTCDataChannel.WriteMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for WriteMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for WriteMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for WriteMode {
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
impl crate::registry::property::Export for WriteMode {
    
}
impl crate::meta::Element for WriteMode {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ChannelState {
    ord: i32
}
impl ChannelState {
    #[doc(alias = "STATE_CONNECTING")]
    #[doc = "Godot enumerator name: `STATE_CONNECTING`"]
    pub const CONNECTING: ChannelState = ChannelState {
        ord: 0i32
    };
    #[doc(alias = "STATE_OPEN")]
    #[doc = "Godot enumerator name: `STATE_OPEN`"]
    pub const OPEN: ChannelState = ChannelState {
        ord: 1i32
    };
    #[doc(alias = "STATE_CLOSING")]
    #[doc = "Godot enumerator name: `STATE_CLOSING`"]
    pub const CLOSING: ChannelState = ChannelState {
        ord: 2i32
    };
    #[doc(alias = "STATE_CLOSED")]
    #[doc = "Godot enumerator name: `STATE_CLOSED`"]
    pub const CLOSED: ChannelState = ChannelState {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ChannelState {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ChannelState") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ChannelState {
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
            Self::CONNECTING => "CONNECTING", Self::OPEN => "OPEN", Self::CLOSING => "CLOSING", Self::CLOSED => "CLOSED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ChannelState::CONNECTING, ChannelState::OPEN, ChannelState::CLOSING, ChannelState::CLOSED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ChannelState >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CONNECTING", "STATE_CONNECTING", ChannelState::CONNECTING), crate::meta::inspect::EnumConstant::new("OPEN", "STATE_OPEN", ChannelState::OPEN), crate::meta::inspect::EnumConstant::new("CLOSING", "STATE_CLOSING", ChannelState::CLOSING), crate::meta::inspect::EnumConstant::new("CLOSED", "STATE_CLOSED", ChannelState::CLOSED)]
        }
    }
}
impl crate::meta::GodotConvert for ChannelState {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("State Connecting", 0i64), EnumeratorShape::new_int("State Open", 1i64), EnumeratorShape::new_int("State Closing", 2i64), EnumeratorShape::new_int("State Closed", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("WebRTCDataChannel.ChannelState")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ChannelState {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ChannelState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ChannelState {
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
impl crate::registry::property::Export for ChannelState {
    
}
impl crate::meta::Element for ChannelState {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::WebRtcDataChannel;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for WebRtcDataChannel {
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
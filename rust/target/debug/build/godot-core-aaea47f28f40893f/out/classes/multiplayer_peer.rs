#![doc = "Sidecar module for class [`MultiplayerPeer`][crate::classes::MultiplayerPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MultiplayerPeer` enums](https://docs.godotengine.org/en/stable/classes/class_multiplayerpeer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MultiplayerPeer`.\n\nInherits [`PacketPeer`][crate::classes::PacketPeer].\n\nRelated symbols:\n\n* [`multiplayer_peer`][crate::classes::multiplayer_peer]: sidecar module with related enum/flag types\n* [`SignalsOfMultiplayerPeer`][crate::classes::multiplayer_peer::SignalsOfMultiplayerPeer]: signal collection\n\n\nSee also [Godot docs for `MultiplayerPeer`](https://docs.godotengine.org/en/stable/classes/class_multiplayerpeer.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<MultiplayerPeer>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nManages the connection with one or more remote peers acting as server or client and assigning unique IDs to each of them. See also [`MultiplayerAPI`][crate::classes::MultiplayerApi].\n\n**Note:** The [`MultiplayerAPI`][crate::classes::MultiplayerApi] protocol is an implementation detail and isn't meant to be used by non-Godot servers. It may change without notice.\n\n**Note:** When exporting to Android, make sure to enable the `INTERNET` permission in the Android export preset before exporting the project or using one-click deploy. Otherwise, network communication of any kind will be blocked by Android."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MultiplayerPeer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl MultiplayerPeer {
        pub fn set_transfer_channel(&mut self, channel: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (channel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7025usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "set_transfer_channel", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transfer_channel(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7026usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "get_transfer_channel", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_transfer_mode(&mut self, mode: crate::classes::multiplayer_peer::TransferMode,) {
            type CallRet = ();
            type CallParams = (crate::classes::multiplayer_peer::TransferMode,);
            let args = (mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7027usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "set_transfer_mode", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_transfer_mode(&self,) -> crate::classes::multiplayer_peer::TransferMode {
            type CallRet = crate::classes::multiplayer_peer::TransferMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7028usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "get_transfer_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the peer to which packets will be sent.\n\nThe `id` can be one of: `TARGET_PEER_BROADCAST` to send to all connected peers, `TARGET_PEER_SERVER` to send to the peer acting as server, a valid peer ID to send to that specific peer, a negative peer ID to send to all peers except that one. By default, the target peer is `TARGET_PEER_BROADCAST`."]
        pub fn set_target_peer(&mut self, id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7029usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "set_target_peer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the ID of the `MultiplayerPeer` who sent the next available packet. See [`get_available_packet_count`][`crate::classes::PacketPeer::get_available_packet_count`]."]
        pub fn get_packet_peer(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7030usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "get_packet_peer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the channel over which the next available packet was received. See [`get_available_packet_count`][`crate::classes::PacketPeer::get_available_packet_count`]."]
        pub fn get_packet_channel(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7031usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "get_packet_channel", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the transfer mode the remote peer used to send the next available packet. See [`get_available_packet_count`][`crate::classes::PacketPeer::get_available_packet_count`]."]
        pub fn get_packet_mode(&self,) -> crate::classes::multiplayer_peer::TransferMode {
            type CallRet = crate::classes::multiplayer_peer::TransferMode;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7032usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "get_packet_mode", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Waits up to 1 second to receive a new network event."]
        pub fn poll(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7033usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "poll", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Immediately close the multiplayer peer returning to the state [`ConnectionStatus::DISCONNECTED`][`crate::classes::multiplayer_peer::ConnectionStatus::DISCONNECTED`]. Connected peers will be dropped without emitting `peer_disconnected`."]
        pub fn close(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7034usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "close", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Disconnects the given `peer` from this host. If `force` is `true` the `peer_disconnected` signal will not be emitted for this peer."]
        pub(crate) fn disconnect_peer_full(&mut self, peer: i32, force: bool,) {
            type CallRet = ();
            type CallParams = (i32, bool,);
            let args = (peer, force,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7035usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "disconnect_peer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`disconnect_peer_ex`][Self::disconnect_peer_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Disconnects the given `peer` from this host. If `force` is `true` the `peer_disconnected` signal will not be emitted for this peer."]
        #[inline]
        pub fn disconnect_peer(&mut self, peer: i32,) {
            self.disconnect_peer_ex(peer,) . done()
        }
        #[doc = "Disconnects the given `peer` from this host. If `force` is `true` the `peer_disconnected` signal will not be emitted for this peer."]
        #[inline]
        pub fn disconnect_peer_ex < 'ex > (&'ex mut self, peer: i32,) -> ExDisconnectPeer < 'ex > {
            ExDisconnectPeer::new(self, peer,)
        }
        #[doc = "Returns the current state of the connection."]
        pub fn get_connection_status(&self,) -> crate::classes::multiplayer_peer::ConnectionStatus {
            type CallRet = crate::classes::multiplayer_peer::ConnectionStatus;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7036usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "get_connection_status", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the ID of this `MultiplayerPeer`."]
        pub fn get_unique_id(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7037usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "get_unique_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a randomly generated integer that can be used as a network unique ID."]
        pub fn generate_unique_id(&self,) -> u32 {
            type CallRet = u32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7038usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "generate_unique_id", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_refuse_new_connections(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7039usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "set_refuse_new_connections", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_refusing_new_connections(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7040usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "is_refusing_new_connections", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the server can act as a relay in the current configuration. That is, if the higher level [`MultiplayerAPI`][crate::classes::MultiplayerApi] should notify connected clients of other peers, and implement a relay protocol to allow communication between them."]
        pub fn is_server_relay_supported(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7041usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerPeer", "is_server_relay_supported", Some(self.__validated_obj()), args,)
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
        pub const TARGET_PEER_BROADCAST: i32 = 0i32;
        pub const TARGET_PEER_SERVER: i32 = 1i32;
        
    }
    impl crate::obj::GodotClass for MultiplayerPeer {
        type Base = crate::classes::PacketPeer;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("MultiplayerPeer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MultiplayerPeer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PacketPeer > for MultiplayerPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for MultiplayerPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for MultiplayerPeer {
        
    }
    impl std::ops::Deref for MultiplayerPeer {
        type Target = crate::classes::PacketPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MultiplayerPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_MultiplayerPeer__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `MultiplayerPeer` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`MultiplayerPeer::disconnect_peer_ex`][super::MultiplayerPeer::disconnect_peer_ex]."]
#[must_use]
pub struct ExDisconnectPeer < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::MultiplayerPeer, peer: i32, force: bool,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExDisconnectPeer < 'ex > {
    fn new(surround_object: &'ex mut re_export::MultiplayerPeer, peer: i32,) -> Self {
        let force = false;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, peer: peer, force: force,
        }
    }
    #[inline]
    pub fn force(self, force: bool) -> Self {
        Self {
            force: force, .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, peer, force,
        }
        = self;
        re_export::MultiplayerPeer::disconnect_peer_full(surround_object, peer, force,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ConnectionStatus {
    ord: i32
}
impl ConnectionStatus {
    #[doc(alias = "CONNECTION_DISCONNECTED")]
    #[doc = "Godot enumerator name: `CONNECTION_DISCONNECTED`"]
    pub const DISCONNECTED: ConnectionStatus = ConnectionStatus {
        ord: 0i32
    };
    #[doc(alias = "CONNECTION_CONNECTING")]
    #[doc = "Godot enumerator name: `CONNECTION_CONNECTING`"]
    pub const CONNECTING: ConnectionStatus = ConnectionStatus {
        ord: 1i32
    };
    #[doc(alias = "CONNECTION_CONNECTED")]
    #[doc = "Godot enumerator name: `CONNECTION_CONNECTED`"]
    pub const CONNECTED: ConnectionStatus = ConnectionStatus {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for ConnectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ConnectionStatus") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ConnectionStatus {
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
            Self::DISCONNECTED => "DISCONNECTED", Self::CONNECTING => "CONNECTING", Self::CONNECTED => "CONNECTED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ConnectionStatus::DISCONNECTED, ConnectionStatus::CONNECTING, ConnectionStatus::CONNECTED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ConnectionStatus >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISCONNECTED", "CONNECTION_DISCONNECTED", ConnectionStatus::DISCONNECTED), crate::meta::inspect::EnumConstant::new("CONNECTING", "CONNECTION_CONNECTING", ConnectionStatus::CONNECTING), crate::meta::inspect::EnumConstant::new("CONNECTED", "CONNECTION_CONNECTED", ConnectionStatus::CONNECTED)]
        }
    }
}
impl crate::meta::GodotConvert for ConnectionStatus {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Connection Disconnected", 0i64), EnumeratorShape::new_int("Connection Connecting", 1i64), EnumeratorShape::new_int("Connection Connected", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("MultiplayerPeer.ConnectionStatus")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ConnectionStatus {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ConnectionStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ConnectionStatus {
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
impl crate::registry::property::Export for ConnectionStatus {
    
}
impl crate::meta::Element for ConnectionStatus {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TransferMode {
    ord: i32
}
impl TransferMode {
    #[doc(alias = "TRANSFER_MODE_UNRELIABLE")]
    #[doc = "Godot enumerator name: `TRANSFER_MODE_UNRELIABLE`"]
    pub const UNRELIABLE: TransferMode = TransferMode {
        ord: 0i32
    };
    #[doc(alias = "TRANSFER_MODE_UNRELIABLE_ORDERED")]
    #[doc = "Godot enumerator name: `TRANSFER_MODE_UNRELIABLE_ORDERED`"]
    pub const UNRELIABLE_ORDERED: TransferMode = TransferMode {
        ord: 1i32
    };
    #[doc(alias = "TRANSFER_MODE_RELIABLE")]
    #[doc = "Godot enumerator name: `TRANSFER_MODE_RELIABLE`"]
    pub const RELIABLE: TransferMode = TransferMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for TransferMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("TransferMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for TransferMode {
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
            Self::UNRELIABLE => "UNRELIABLE", Self::UNRELIABLE_ORDERED => "UNRELIABLE_ORDERED", Self::RELIABLE => "RELIABLE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[TransferMode::UNRELIABLE, TransferMode::UNRELIABLE_ORDERED, TransferMode::RELIABLE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < TransferMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("UNRELIABLE", "TRANSFER_MODE_UNRELIABLE", TransferMode::UNRELIABLE), crate::meta::inspect::EnumConstant::new("UNRELIABLE_ORDERED", "TRANSFER_MODE_UNRELIABLE_ORDERED", TransferMode::UNRELIABLE_ORDERED), crate::meta::inspect::EnumConstant::new("RELIABLE", "TRANSFER_MODE_RELIABLE", TransferMode::RELIABLE)]
        }
    }
}
impl crate::meta::GodotConvert for TransferMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Transfer Mode Unreliable", 0i64), EnumeratorShape::new_int("Transfer Mode Unreliable Ordered", 1i64), EnumeratorShape::new_int("Transfer Mode Reliable", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("MultiplayerPeer.TransferMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for TransferMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for TransferMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for TransferMode {
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
impl crate::registry::property::Export for TransferMode {
    
}
impl crate::meta::Element for TransferMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::MultiplayerPeer;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`MultiplayerPeer`][crate::classes::MultiplayerPeer] class."]
    pub struct SignalsOfMultiplayerPeer < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfMultiplayerPeer < 'c, C > {
        #[doc = "Signature: `(id: i64)`"]
        pub fn peer_connected(&mut self) -> SigPeerConnected < 'c, C > {
            SigPeerConnected {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "peer_connected")
            }
        }
        #[doc = "Signature: `(id: i64)`"]
        pub fn peer_disconnected(&mut self) -> SigPeerDisconnected < 'c, C > {
            SigPeerDisconnected {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "peer_disconnected")
            }
        }
    }
    type TypedSigPeerConnected < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigPeerConnected < 'c, C: WithSignals > {
        typed: TypedSigPeerConnected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPeerConnected < 'c, C > {
        pub fn emit(&mut self, id: i64,) {
            self.typed.emit_tuple((id,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPeerConnected < 'c, C > {
        type Target = TypedSigPeerConnected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPeerConnected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigPeerDisconnected < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigPeerDisconnected < 'c, C: WithSignals > {
        typed: TypedSigPeerDisconnected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPeerDisconnected < 'c, C > {
        pub fn emit(&mut self, id: i64,) {
            self.typed.emit_tuple((id,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPeerDisconnected < 'c, C > {
        type Target = TypedSigPeerDisconnected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPeerDisconnected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for MultiplayerPeer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfMultiplayerPeer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfMultiplayerPeer < 'c, C > {
        type Target = < < MultiplayerPeer as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = MultiplayerPeer;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfMultiplayerPeer < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = MultiplayerPeer;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
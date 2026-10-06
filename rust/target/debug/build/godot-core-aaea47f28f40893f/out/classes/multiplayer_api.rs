#![doc = "Sidecar module for class [`MultiplayerApi`][crate::classes::MultiplayerApi].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MultiplayerAPI` enums](https://docs.godotengine.org/en/stable/classes/class_multiplayerapi.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MultiplayerAPI`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`multiplayer_api`][crate::classes::multiplayer_api]: sidecar module with related enum/flag types\n* [`SignalsOfMultiplayerApi`][crate::classes::multiplayer_api::SignalsOfMultiplayerApi]: signal collection\n\n\nSee also [Godot docs for `MultiplayerAPI`](https://docs.godotengine.org/en/stable/classes/class_multiplayerapi.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<MultiplayerApi>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nBase class for high-level multiplayer API implementations. See also [`MultiplayerPeer`][crate::classes::MultiplayerPeer].\n\nBy default, [`SceneTree`][crate::classes::SceneTree] has a reference to an implementation of this class and uses it to provide multiplayer capabilities (i.e. RPCs) across the whole scene.\n\nIt is possible to override the MultiplayerAPI instance used by specific tree branches by calling the [`set_multiplayer`][`crate::classes::SceneTree::set_multiplayer`] method, effectively allowing to run both client and server in the same scene.\n\nIt is also possible to extend or replace the default implementation via scripting or native extensions. See [`MultiplayerAPIExtension`][crate::classes::MultiplayerApiExtension] for details about extensions, [`SceneMultiplayer`][crate::classes::SceneMultiplayer] for the details about the default implementation."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MultiplayerApi {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl MultiplayerApi {
        #[doc = "Returns `true` if there is a \\[member multiplayer_peer] set."]
        pub fn has_multiplayer_peer(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9515usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerApi", "has_multiplayer_peer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_multiplayer_peer(&self,) -> Option < Gd < crate::classes::MultiplayerPeer > > {
            type CallRet = Option < Gd < crate::classes::MultiplayerPeer > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9516usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerApi", "get_multiplayer_peer", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_multiplayer_peer(&mut self, peer: impl AsArg < Option < Gd < crate::classes::MultiplayerPeer >> >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::MultiplayerPeer > > >,);
            let args = (peer.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9517usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerApi", "set_multiplayer_peer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the unique peer ID of this MultiplayerAPI's \\[member multiplayer_peer]."]
        pub fn get_unique_id(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9518usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerApi", "get_unique_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if this MultiplayerAPI's \\[member multiplayer_peer] is valid and in server mode (listening for connections)."]
        pub fn is_server(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9519usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerApi", "is_server", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the sender's peer ID for the RPC currently being executed.\n\n**Note:** This method returns `0` when called outside of an RPC. As such, the original peer ID may be lost when code execution is delayed (such as with GDScript's `await` keyword)."]
        pub fn get_remote_sender_id(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9520usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerApi", "get_remote_sender_id", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Method used for polling the MultiplayerAPI. You only need to worry about this if you set \\[member SceneTree.multiplayer_poll] to `false`. By default, [`SceneTree`][crate::classes::SceneTree] will poll its MultiplayerAPI(s) for you.\n\n**Note:** This method results in RPCs being called, so they will be executed in the same context of this function (e.g. `_process`, `physics`, `Thread`)."]
        pub fn poll(&mut self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9521usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerApi", "poll", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sends an RPC to the target `peer`. The given `method` will be called on the remote `object` with the provided `arguments`. The RPC may also be called locally depending on the implementation and RPC configuration. See [`rpc`][`crate::classes::Node::rpc`] and [`rpc_config`][`crate::classes::Node::rpc_config`].\n\n**Note:** Prefer using [`rpc`][`crate::classes::Node::rpc`], [`rpc_id`][`crate::classes::Node::rpc_id`], or `my_method.rpc(peer, arg1, arg2, ...)` (in GDScript), since they are faster. This method is mostly useful in conjunction with [`MultiplayerAPIExtension`][crate::classes::MultiplayerApiExtension] when extending or replacing the multiplayer capabilities."]
        pub(crate) fn rpc_full(&mut self, peer: i32, object: CowArg < Option < Gd < crate::classes::Object > > >, method: CowArg < StringName >, arguments: RefArg < AnyArray >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, 'a2, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::Object > > >, CowArg < 'a1, StringName >, RefArg < 'a2, AnyArray >,);
            let args = (peer, object, method, arguments,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9522usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerApi", "rpc", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`rpc_ex`][Self::rpc_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sends an RPC to the target `peer`. The given `method` will be called on the remote `object` with the provided `arguments`. The RPC may also be called locally depending on the implementation and RPC configuration. See [`rpc`][`crate::classes::Node::rpc`] and [`rpc_config`][`crate::classes::Node::rpc_config`].\n\n**Note:** Prefer using [`rpc`][`crate::classes::Node::rpc`], [`rpc_id`][`crate::classes::Node::rpc_id`], or `my_method.rpc(peer, arg1, arg2, ...)` (in GDScript), since they are faster. This method is mostly useful in conjunction with [`MultiplayerAPIExtension`][crate::classes::MultiplayerApiExtension] when extending or replacing the multiplayer capabilities."]
        #[inline]
        pub fn rpc(&mut self, peer: i32, object: impl AsArg < Option < Gd < crate::classes::Object >> >, method: impl AsArg < StringName >,) -> crate::global::Error {
            self.rpc_ex(peer, object, method,) . done()
        }
        #[doc = "Sends an RPC to the target `peer`. The given `method` will be called on the remote `object` with the provided `arguments`. The RPC may also be called locally depending on the implementation and RPC configuration. See [`rpc`][`crate::classes::Node::rpc`] and [`rpc_config`][`crate::classes::Node::rpc_config`].\n\n**Note:** Prefer using [`rpc`][`crate::classes::Node::rpc`], [`rpc_id`][`crate::classes::Node::rpc_id`], or `my_method.rpc(peer, arg1, arg2, ...)` (in GDScript), since they are faster. This method is mostly useful in conjunction with [`MultiplayerAPIExtension`][crate::classes::MultiplayerApiExtension] when extending or replacing the multiplayer capabilities."]
        #[inline]
        pub fn rpc_ex < 'ex > (&'ex mut self, peer: i32, object: impl AsArg < Option < Gd < crate::classes::Object >> > + 'ex, method: impl AsArg < StringName > + 'ex,) -> ExRpc < 'ex > {
            ExRpc::new(self, peer, object, method,)
        }
        #[doc = "Notifies the MultiplayerAPI of a new `configuration` for the given `object`. This method is used internally by [`SceneTree`][crate::classes::SceneTree] to configure the root path for this MultiplayerAPI (passing `null` and a valid [`NodePath`][crate::builtin::NodePath] as `configuration`). This method can be further used by MultiplayerAPI implementations to provide additional features, refer to specific implementation (e.g. [`SceneMultiplayer`][crate::classes::SceneMultiplayer]) for details on how they use it.\n\n**Note:** This method is mostly relevant when extending or overriding the MultiplayerAPI behavior via [`MultiplayerAPIExtension`][crate::classes::MultiplayerApiExtension]."]
        pub fn object_configuration_add(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >, configuration: &Variant,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >, RefArg < 'a1, Variant >,);
            let args = (object.into_arg(), RefArg::new(configuration),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9523usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerApi", "object_configuration_add", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Notifies the MultiplayerAPI to remove a `configuration` for the given `object`. This method is used internally by [`SceneTree`][crate::classes::SceneTree] to configure the root path for this MultiplayerAPI (passing `null` and an empty [`NodePath`][crate::builtin::NodePath] as `configuration`). This method can be further used by MultiplayerAPI implementations to provide additional features, refer to specific implementation (e.g. [`SceneMultiplayer`][crate::classes::SceneMultiplayer]) for details on how they use it.\n\n**Note:** This method is mostly relevant when extending or overriding the MultiplayerAPI behavior via [`MultiplayerAPIExtension`][crate::classes::MultiplayerApiExtension]."]
        pub fn object_configuration_remove(&mut self, object: impl AsArg < Option < Gd < crate::classes::Object >> >, configuration: &Variant,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::Object > > >, RefArg < 'a1, Variant >,);
            let args = (object.into_arg(), RefArg::new(configuration),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9524usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerApi", "object_configuration_remove", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the peer IDs of all connected peers of this MultiplayerAPI's \\[member multiplayer_peer]."]
        pub fn get_peers(&self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9525usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerApi", "get_peers", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the default MultiplayerAPI implementation class. This method can be used by modules and extensions to configure which implementation will be used by [`SceneTree`][crate::classes::SceneTree] when the engine starts."]
        pub fn set_default_interface(interface_name: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (interface_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9526usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerApi", "set_default_interface", None, args,)
            }
        }
        #[doc = "Returns the default MultiplayerAPI implementation class name. This is usually `\"SceneMultiplayer\"` when [`SceneMultiplayer`][crate::classes::SceneMultiplayer] is available. See [`set_default_interface`][`crate::classes::MultiplayerApi::set_default_interface`]."]
        pub fn get_default_interface() -> StringName {
            type CallRet = StringName;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9527usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerApi", "get_default_interface", None, args,)
            }
        }
        #[doc = "Returns a new instance of the default MultiplayerAPI."]
        pub fn create_default_interface() -> Option < Gd < crate::classes::MultiplayerApi > > {
            type CallRet = Option < Gd < crate::classes::MultiplayerApi > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9528usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "MultiplayerApi", "create_default_interface", None, args,)
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
    impl crate::obj::GodotClass for MultiplayerApi {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("MultiplayerAPI"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MultiplayerApi {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for MultiplayerApi {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for MultiplayerApi {
        
    }
    impl std::ops::Deref for MultiplayerApi {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MultiplayerApi {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_MultiplayerApi__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `MultiplayerApi` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`MultiplayerApi::rpc_ex`][super::MultiplayerApi::rpc_ex]."]
#[must_use]
pub struct ExRpc < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::MultiplayerApi, peer: i32, object: CowArg < 'ex, Option < Gd < crate::classes::Object > > >, method: CowArg < 'ex, StringName >, arguments: CowArg < 'ex, AnyArray >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExRpc < 'ex > {
    fn new(surround_object: &'ex mut re_export::MultiplayerApi, peer: i32, object: impl AsArg < Option < Gd < crate::classes::Object >> > + 'ex, method: impl AsArg < StringName > + 'ex,) -> Self {
        let arguments = AnyArray::new_untyped();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, peer: peer, object: object.into_arg(), method: method.into_arg(), arguments: CowArg::Owned(arguments),
        }
    }
    #[inline]
    pub fn arguments(self, arguments: &'ex AnyArray) -> Self {
        Self {
            arguments: CowArg::Borrowed(arguments), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, peer, object, method, arguments,
        }
        = self;
        re_export::MultiplayerApi::rpc_full(surround_object, peer, object, method, arguments.cow_as_arg(),)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[doc = "Godot enum name: `RPCMode`."]
pub struct RpcMode {
    ord: i32
}
impl RpcMode {
    #[doc(alias = "RPC_MODE_DISABLED")]
    #[doc = "Godot enumerator name: `RPC_MODE_DISABLED`"]
    pub const DISABLED: RpcMode = RpcMode {
        ord: 0i32
    };
    #[doc(alias = "RPC_MODE_ANY_PEER")]
    #[doc = "Godot enumerator name: `RPC_MODE_ANY_PEER`"]
    pub const ANY_PEER: RpcMode = RpcMode {
        ord: 1i32
    };
    #[doc(alias = "RPC_MODE_AUTHORITY")]
    #[doc = "Godot enumerator name: `RPC_MODE_AUTHORITY`"]
    pub const AUTHORITY: RpcMode = RpcMode {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for RpcMode {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("RpcMode") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for RpcMode {
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
            Self::DISABLED => "DISABLED", Self::ANY_PEER => "ANY_PEER", Self::AUTHORITY => "AUTHORITY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[RpcMode::DISABLED, RpcMode::ANY_PEER, RpcMode::AUTHORITY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < RpcMode >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("DISABLED", "RPC_MODE_DISABLED", RpcMode::DISABLED), crate::meta::inspect::EnumConstant::new("ANY_PEER", "RPC_MODE_ANY_PEER", RpcMode::ANY_PEER), crate::meta::inspect::EnumConstant::new("AUTHORITY", "RPC_MODE_AUTHORITY", RpcMode::AUTHORITY)]
        }
    }
}
impl crate::meta::GodotConvert for RpcMode {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Rpc Mode Disabled", 0i64), EnumeratorShape::new_int("Rpc Mode Any Peer", 1i64), EnumeratorShape::new_int("Rpc Mode Authority", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("MultiplayerAPI.RPCMode")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for RpcMode {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for RpcMode {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for RpcMode {
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
impl crate::registry::property::Export for RpcMode {
    
}
impl crate::meta::Element for RpcMode {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::MultiplayerApi;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`MultiplayerApi`][crate::classes::MultiplayerApi] class."]
    pub struct SignalsOfMultiplayerApi < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfMultiplayerApi < 'c, C > {
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
        #[doc = "Signature: `()`"]
        pub fn connected_to_server(&mut self) -> SigConnectedToServer < 'c, C > {
            SigConnectedToServer {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "connected_to_server")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn connection_failed(&mut self) -> SigConnectionFailed < 'c, C > {
            SigConnectionFailed {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "connection_failed")
            }
        }
        #[doc = "Signature: `()`"]
        pub fn server_disconnected(&mut self) -> SigServerDisconnected < 'c, C > {
            SigServerDisconnected {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "server_disconnected")
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
    type TypedSigConnectedToServer < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigConnectedToServer < 'c, C: WithSignals > {
        typed: TypedSigConnectedToServer < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigConnectedToServer < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigConnectedToServer < 'c, C > {
        type Target = TypedSigConnectedToServer < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigConnectedToServer < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigConnectionFailed < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigConnectionFailed < 'c, C: WithSignals > {
        typed: TypedSigConnectionFailed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigConnectionFailed < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigConnectionFailed < 'c, C > {
        type Target = TypedSigConnectionFailed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigConnectionFailed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigServerDisconnected < 'c, C > = TypedSignal < 'c, C, () >;
    pub struct SigServerDisconnected < 'c, C: WithSignals > {
        typed: TypedSigServerDisconnected < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigServerDisconnected < 'c, C > {
        pub fn emit(&mut self,) {
            self.typed.emit_tuple(());
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigServerDisconnected < 'c, C > {
        type Target = TypedSigServerDisconnected < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigServerDisconnected < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for MultiplayerApi {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfMultiplayerApi < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfMultiplayerApi < 'c, C > {
        type Target = < < MultiplayerApi as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = MultiplayerApi;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfMultiplayerApi < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = MultiplayerApi;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
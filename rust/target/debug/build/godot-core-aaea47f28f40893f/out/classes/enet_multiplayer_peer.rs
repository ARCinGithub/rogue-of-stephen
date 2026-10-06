#![doc = "Sidecar module for class [`ENetMultiplayerPeer`][crate::classes::ENetMultiplayerPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ENetMultiplayerPeer` enums](https://docs.godotengine.org/en/stable/classes/class_enetmultiplayerpeer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `ENetMultiplayerPeer`.\n\nInherits [`MultiplayerPeer`][crate::classes::MultiplayerPeer].\n\nRelated symbols:\n\n* [`enet_multiplayer_peer`][crate::classes::enet_multiplayer_peer]: sidecar module with related enum/flag types\n* [`IENetMultiplayerPeer`][crate::classes::IENetMultiplayerPeer]: virtual methods\n\n\nSee also [Godot docs for `ENetMultiplayerPeer`](https://docs.godotengine.org/en/stable/classes/class_enetmultiplayerpeer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`ENetMultiplayerPeer::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nA MultiplayerPeer implementation that should be passed to \\[member MultiplayerAPI.multiplayer_peer] after being initialized as either a client, server, or mesh. Events can then be handled by connecting to [`MultiplayerAPI`][crate::classes::MultiplayerApi] signals. See [`ENetConnection`][crate::classes::ENetConnection] for more information on the ENet library wrapper.\n\n**Note:** ENet only uses UDP, not TCP. When forwarding the server port to make your server accessible on the public Internet, you only need to forward the server port in UDP. You can use the [`UPNP`][crate::classes::Upnp] class to try to forward the server port automatically when starting the server."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct ENetMultiplayerPeer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`ENetMultiplayerPeer`][crate::classes::ENetMultiplayerPeer].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IMultiplayerPeer`~~ > ~~`IPacketPeer`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `ENetMultiplayerPeer` methods](https://docs.godotengine.org/en/stable/classes/class_enetmultiplayerpeer.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IENetMultiplayerPeer: crate::obj::GodotClass < Base = ENetMultiplayerPeer > + crate::private::You_forgot_the_attribute__godot_api {
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
    }
    impl ENetMultiplayerPeer {
        #[doc = "Create server that listens to connections via `port`. The port needs to be an available, unused port between 0 and 65535. Note that ports below 1024 are privileged and may require elevated permissions depending on the platform. To change the interface the server listens on, use [`set_bind_ip`][`crate::classes::ENetMultiplayerPeer::set_bind_ip`]. The default IP is the wildcard `\"*\"`, which listens on all available interfaces. `max_clients` is the maximum number of clients that are allowed at once, any number up to 4095 may be used, although the achievable number of simultaneous clients may be far lower and depends on the application. For additional details on the bandwidth parameters, see [`create_client`][`crate::classes::ENetMultiplayerPeer::create_client`]. Returns [`Error::OK`][`crate::global::Error::OK`] if a server was created, [`Error::ERR_ALREADY_IN_USE`][`crate::global::Error::ERR_ALREADY_IN_USE`] if this ENetMultiplayerPeer instance already has an open connection (in which case you need to call [`close`][`crate::classes::MultiplayerPeer::close`] first) or [`Error::ERR_CANT_CREATE`][`crate::global::Error::ERR_CANT_CREATE`] if the server could not be created."]
        pub(crate) fn create_server_full(&mut self, port: i32, max_clients: i32, max_channels: i32, in_bandwidth: i32, out_bandwidth: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (i32, i32, i32, i32, i32,);
            let args = (port, max_clients, max_channels, in_bandwidth, out_bandwidth,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10006usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetMultiplayerPeer", "create_server", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_server_ex`][Self::create_server_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Create server that listens to connections via `port`. The port needs to be an available, unused port between 0 and 65535. Note that ports below 1024 are privileged and may require elevated permissions depending on the platform. To change the interface the server listens on, use [`set_bind_ip`][`crate::classes::ENetMultiplayerPeer::set_bind_ip`]. The default IP is the wildcard `\"*\"`, which listens on all available interfaces. `max_clients` is the maximum number of clients that are allowed at once, any number up to 4095 may be used, although the achievable number of simultaneous clients may be far lower and depends on the application. For additional details on the bandwidth parameters, see [`create_client`][`crate::classes::ENetMultiplayerPeer::create_client`]. Returns [`Error::OK`][`crate::global::Error::OK`] if a server was created, [`Error::ERR_ALREADY_IN_USE`][`crate::global::Error::ERR_ALREADY_IN_USE`] if this ENetMultiplayerPeer instance already has an open connection (in which case you need to call [`close`][`crate::classes::MultiplayerPeer::close`] first) or [`Error::ERR_CANT_CREATE`][`crate::global::Error::ERR_CANT_CREATE`] if the server could not be created."]
        #[inline]
        pub fn create_server(&mut self, port: i32,) -> crate::global::Error {
            self.create_server_ex(port,) . done()
        }
        #[doc = "Create server that listens to connections via `port`. The port needs to be an available, unused port between 0 and 65535. Note that ports below 1024 are privileged and may require elevated permissions depending on the platform. To change the interface the server listens on, use [`set_bind_ip`][`crate::classes::ENetMultiplayerPeer::set_bind_ip`]. The default IP is the wildcard `\"*\"`, which listens on all available interfaces. `max_clients` is the maximum number of clients that are allowed at once, any number up to 4095 may be used, although the achievable number of simultaneous clients may be far lower and depends on the application. For additional details on the bandwidth parameters, see [`create_client`][`crate::classes::ENetMultiplayerPeer::create_client`]. Returns [`Error::OK`][`crate::global::Error::OK`] if a server was created, [`Error::ERR_ALREADY_IN_USE`][`crate::global::Error::ERR_ALREADY_IN_USE`] if this ENetMultiplayerPeer instance already has an open connection (in which case you need to call [`close`][`crate::classes::MultiplayerPeer::close`] first) or [`Error::ERR_CANT_CREATE`][`crate::global::Error::ERR_CANT_CREATE`] if the server could not be created."]
        #[inline]
        pub fn create_server_ex < 'ex > (&'ex mut self, port: i32,) -> ExCreateServer < 'ex > {
            ExCreateServer::new(self, port,)
        }
        #[doc = "Create client that connects to a server at `address` using specified `port`. The given address needs to be either a fully qualified domain name (e.g. `\"www.example.com\"`) or an IP address in IPv4 or IPv6 format (e.g. `\"192.168.1.1\"`). The `port` is the port the server is listening on. The `channel_count` parameter can be used to specify the number of ENet channels allocated for the connection. The `in_bandwidth` and `out_bandwidth` parameters can be used to limit the incoming and outgoing bandwidth to the given number of bytes per second. The default of 0 means unlimited bandwidth. Note that ENet will strategically drop packets on specific sides of a connection between peers to ensure the peer's bandwidth is not overwhelmed. The bandwidth parameters also determine the window size of a connection which limits the amount of reliable packets that may be in transit at any given time. Returns [`Error::OK`][`crate::global::Error::OK`] if a client was created, [`Error::ERR_ALREADY_IN_USE`][`crate::global::Error::ERR_ALREADY_IN_USE`] if this ENetMultiplayerPeer instance already has an open connection (in which case you need to call [`close`][`crate::classes::MultiplayerPeer::close`] first) or [`Error::ERR_CANT_CREATE`][`crate::global::Error::ERR_CANT_CREATE`] if the client could not be created. If `local_port` is specified, the client will also listen to the given port; this is useful for some NAT traversal techniques."]
        pub(crate) fn create_client_full(&mut self, address: CowArg < GString >, port: i32, channel_count: i32, in_bandwidth: i32, out_bandwidth: i32, local_port: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32, i32, i32, i32, i32,);
            let args = (address, port, channel_count, in_bandwidth, out_bandwidth, local_port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10007usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetMultiplayerPeer", "create_client", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_client_ex`][Self::create_client_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Create client that connects to a server at `address` using specified `port`. The given address needs to be either a fully qualified domain name (e.g. `\"www.example.com\"`) or an IP address in IPv4 or IPv6 format (e.g. `\"192.168.1.1\"`). The `port` is the port the server is listening on. The `channel_count` parameter can be used to specify the number of ENet channels allocated for the connection. The `in_bandwidth` and `out_bandwidth` parameters can be used to limit the incoming and outgoing bandwidth to the given number of bytes per second. The default of 0 means unlimited bandwidth. Note that ENet will strategically drop packets on specific sides of a connection between peers to ensure the peer's bandwidth is not overwhelmed. The bandwidth parameters also determine the window size of a connection which limits the amount of reliable packets that may be in transit at any given time. Returns [`Error::OK`][`crate::global::Error::OK`] if a client was created, [`Error::ERR_ALREADY_IN_USE`][`crate::global::Error::ERR_ALREADY_IN_USE`] if this ENetMultiplayerPeer instance already has an open connection (in which case you need to call [`close`][`crate::classes::MultiplayerPeer::close`] first) or [`Error::ERR_CANT_CREATE`][`crate::global::Error::ERR_CANT_CREATE`] if the client could not be created. If `local_port` is specified, the client will also listen to the given port; this is useful for some NAT traversal techniques."]
        #[inline]
        pub fn create_client(&mut self, address: impl AsArg < GString >, port: i32,) -> crate::global::Error {
            self.create_client_ex(address, port,) . done()
        }
        #[doc = "Create client that connects to a server at `address` using specified `port`. The given address needs to be either a fully qualified domain name (e.g. `\"www.example.com\"`) or an IP address in IPv4 or IPv6 format (e.g. `\"192.168.1.1\"`). The `port` is the port the server is listening on. The `channel_count` parameter can be used to specify the number of ENet channels allocated for the connection. The `in_bandwidth` and `out_bandwidth` parameters can be used to limit the incoming and outgoing bandwidth to the given number of bytes per second. The default of 0 means unlimited bandwidth. Note that ENet will strategically drop packets on specific sides of a connection between peers to ensure the peer's bandwidth is not overwhelmed. The bandwidth parameters also determine the window size of a connection which limits the amount of reliable packets that may be in transit at any given time. Returns [`Error::OK`][`crate::global::Error::OK`] if a client was created, [`Error::ERR_ALREADY_IN_USE`][`crate::global::Error::ERR_ALREADY_IN_USE`] if this ENetMultiplayerPeer instance already has an open connection (in which case you need to call [`close`][`crate::classes::MultiplayerPeer::close`] first) or [`Error::ERR_CANT_CREATE`][`crate::global::Error::ERR_CANT_CREATE`] if the client could not be created. If `local_port` is specified, the client will also listen to the given port; this is useful for some NAT traversal techniques."]
        #[inline]
        pub fn create_client_ex < 'ex > (&'ex mut self, address: impl AsArg < GString > + 'ex, port: i32,) -> ExCreateClient < 'ex > {
            ExCreateClient::new(self, address, port,)
        }
        #[doc = "Initialize this [`MultiplayerPeer`][crate::classes::MultiplayerPeer] in mesh mode. The provided `unique_id` will be used as the local peer network unique ID once assigned as the \\[member MultiplayerAPI.multiplayer_peer]. In the mesh configuration you will need to set up each new peer manually using [`ENetConnection`][crate::classes::ENetConnection] before calling [`add_mesh_peer`][`crate::classes::ENetMultiplayerPeer::add_mesh_peer`]. While this technique is more advanced, it allows for better control over the connection process (e.g. when dealing with NAT punch-through) and for better distribution of the network load (which would otherwise be more taxing on the server)."]
        pub fn create_mesh(&mut self, unique_id: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (i32,);
            let args = (unique_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10008usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetMultiplayerPeer", "create_mesh", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Add a new remote peer with the given `peer_id` connected to the given `host`.\n\n**Note:** The `host` must have exactly one peer in the [`PeerState::CONNECTED`][`crate::classes::enet_packet_peer::PeerState::CONNECTED`] state."]
        pub fn add_mesh_peer(&mut self, peer_id: i32, host: impl AsArg < Option < Gd < crate::classes::ENetConnection >> >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (i32, CowArg < 'a0, Option < Gd < crate::classes::ENetConnection > > >,);
            let args = (peer_id, host.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10009usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetMultiplayerPeer", "add_mesh_peer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "The IP used when creating a server. This is set to the wildcard `\"*\"` by default, which binds to all available interfaces. The given IP needs to be in IPv4 or IPv6 address format, for example: `\"192.168.1.1\"`."]
        pub fn set_bind_ip(&mut self, ip: impl AsArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (ip.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10010usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetMultiplayerPeer", "set_bind_ip", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_host(&self,) -> Option < Gd < crate::classes::ENetConnection > > {
            type CallRet = Option < Gd < crate::classes::ENetConnection > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10011usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetMultiplayerPeer", "get_host", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`ENetPacketPeer`][crate::classes::ENetPacketPeer] associated to the given `id`."]
        pub fn get_peer(&self, id: i32,) -> Option < Gd < crate::classes::ENetPacketPeer > > {
            type CallRet = Option < Gd < crate::classes::ENetPacketPeer > >;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(10012usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "ENetMultiplayerPeer", "get_peer", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for ENetMultiplayerPeer {
        type Base = crate::classes::MultiplayerPeer;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("ENetMultiplayerPeer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for ENetMultiplayerPeer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::MultiplayerPeer > for ENetMultiplayerPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PacketPeer > for ENetMultiplayerPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for ENetMultiplayerPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for ENetMultiplayerPeer {
        
    }
    impl crate::obj::cap::GodotDefault for ENetMultiplayerPeer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for ENetMultiplayerPeer {
        type Target = crate::classes::MultiplayerPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for ENetMultiplayerPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`ENetMultiplayerPeer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_ENetMultiplayerPeer__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::ENetMultiplayerPeer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::MultiplayerPeer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::PacketPeer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`ENetMultiplayerPeer::create_server_ex`][super::ENetMultiplayerPeer::create_server_ex]."]
#[must_use]
pub struct ExCreateServer < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::ENetMultiplayerPeer, port: i32, max_clients: i32, max_channels: i32, in_bandwidth: i32, out_bandwidth: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateServer < 'ex > {
    fn new(surround_object: &'ex mut re_export::ENetMultiplayerPeer, port: i32,) -> Self {
        let max_clients = 32i32;
        let max_channels = 0i32;
        let in_bandwidth = 0i32;
        let out_bandwidth = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, port: port, max_clients: max_clients, max_channels: max_channels, in_bandwidth: in_bandwidth, out_bandwidth: out_bandwidth,
        }
    }
    #[inline]
    pub fn max_clients(self, max_clients: i32) -> Self {
        Self {
            max_clients: max_clients, .. self
        }
    }
    #[inline]
    pub fn max_channels(self, max_channels: i32) -> Self {
        Self {
            max_channels: max_channels, .. self
        }
    }
    #[inline]
    pub fn in_bandwidth(self, in_bandwidth: i32) -> Self {
        Self {
            in_bandwidth: in_bandwidth, .. self
        }
    }
    #[inline]
    pub fn out_bandwidth(self, out_bandwidth: i32) -> Self {
        Self {
            out_bandwidth: out_bandwidth, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, port, max_clients, max_channels, in_bandwidth, out_bandwidth,
        }
        = self;
        re_export::ENetMultiplayerPeer::create_server_full(surround_object, port, max_clients, max_channels, in_bandwidth, out_bandwidth,)
    }
}
#[doc = "Default-param extender for [`ENetMultiplayerPeer::create_client_ex`][super::ENetMultiplayerPeer::create_client_ex]."]
#[must_use]
pub struct ExCreateClient < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::ENetMultiplayerPeer, address: CowArg < 'ex, GString >, port: i32, channel_count: i32, in_bandwidth: i32, out_bandwidth: i32, local_port: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateClient < 'ex > {
    fn new(surround_object: &'ex mut re_export::ENetMultiplayerPeer, address: impl AsArg < GString > + 'ex, port: i32,) -> Self {
        let channel_count = 0i32;
        let in_bandwidth = 0i32;
        let out_bandwidth = 0i32;
        let local_port = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, address: address.into_arg(), port: port, channel_count: channel_count, in_bandwidth: in_bandwidth, out_bandwidth: out_bandwidth, local_port: local_port,
        }
    }
    #[inline]
    pub fn channel_count(self, channel_count: i32) -> Self {
        Self {
            channel_count: channel_count, .. self
        }
    }
    #[inline]
    pub fn in_bandwidth(self, in_bandwidth: i32) -> Self {
        Self {
            in_bandwidth: in_bandwidth, .. self
        }
    }
    #[inline]
    pub fn out_bandwidth(self, out_bandwidth: i32) -> Self {
        Self {
            out_bandwidth: out_bandwidth, .. self
        }
    }
    #[inline]
    pub fn local_port(self, local_port: i32) -> Self {
        Self {
            local_port: local_port, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, address, port, channel_count, in_bandwidth, out_bandwidth, local_port,
        }
        = self;
        re_export::ENetMultiplayerPeer::create_client_full(surround_object, address, port, channel_count, in_bandwidth, out_bandwidth, local_port,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::ENetMultiplayerPeer;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::multiplayer_peer::SignalsOfMultiplayerPeer;
    impl WithSignals for ENetMultiplayerPeer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfMultiplayerPeer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
}
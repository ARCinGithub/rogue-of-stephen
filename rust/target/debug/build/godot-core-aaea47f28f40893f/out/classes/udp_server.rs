#![doc = "Sidecar module for class [`UdpServer`][crate::classes::UdpServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `UDPServer` enums](https://docs.godotengine.org/en/stable/classes/class_udpserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `UDPServer`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`udp_server`][crate::classes::udp_server]: sidecar module with related enum/flag types\n* [`IUdpServer`][crate::classes::IUdpServer]: virtual methods\n\n\nSee also [Godot docs for `UDPServer`](https://docs.godotengine.org/en/stable/classes/class_udpserver.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`UdpServer::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nA simple server that opens a UDP socket and returns connected [`PacketPeerUDP`][crate::classes::PacketPeerUdp] upon receiving new packets. See also [`connect_to_host`][`crate::classes::PacketPeerUdp::connect_to_host`].\n\nAfter starting the server ([`listen`][`crate::classes::UdpServer::listen`]), you will need to [`poll`][`crate::classes::UdpServer::poll`] it at regular intervals (e.g. inside [`process`][`crate::classes::INode::process`]) for it to process new packets, delivering them to the appropriate [`PacketPeerUDP`][crate::classes::PacketPeerUdp], and taking new connections.\n\nBelow a small example of how it can be used:\n\n\n```gdscript\n# server_node.gd\nclass_name ServerNode\nextends Node\n\nvar server = UDPServer.new()\nvar peers = []\n\nfunc _ready():\n\tserver.listen(4242)\n\nfunc _process(delta):\n\tserver.poll() # Important!\n\tif server.is_connection_available():\n\t\tvar peer = server.take_connection()\n\t\tvar packet = peer.get_packet()\n\t\tprint(\"Accepted peer: %s:%s\" % [peer.get_packet_ip(), peer.get_packet_port()])\n\t\tprint(\"Received data: %s\" % [packet.get_string_from_utf8()])\n\t\t# Reply so it knows we received the message.\n\t\tpeer.put_packet(packet)\n\t\t# Keep a reference so we can keep contacting the remote peer.\n\t\tpeers.append(peer)\n\n\tfor i in range(0, peers.size()):\n\t\tpass # Do something with the connected peers.\n```\n\n\n\n```gdscript\n# client_node.gd\nclass_name ClientNode\nextends Node\n\nvar udp = PacketPeerUDP.new()\nvar connected = false\n\nfunc _ready():\n\tudp.connect_to_host(\"127.0.0.1\", 4242)\n\nfunc _process(delta):\n\tif !connected:\n\t\t# Try to contact server\n\t\tudp.put_packet(\"The answer is... 42!\".to_utf8_buffer())\n\tif udp.get_available_packet_count() > 0:\n\t\tprint(\"Connected: %s\" % udp.get_packet().get_string_from_utf8())\n\t\tconnected = true\n```\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct UdpServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`UdpServer`][crate::classes::UdpServer].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `UDPServer` methods](https://docs.godotengine.org/en/stable/classes/class_udpserver.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IUdpServer: crate::obj::GodotClass < Base = UdpServer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl UdpServer {
        #[doc = "Starts the server by opening a UDP socket listening on the given `port`. You can optionally specify a `bind_address` to only listen for packets sent to that address. See also [`bind`][`crate::classes::PacketPeerUdp::bind`]."]
        pub(crate) fn listen_full(&mut self, port: u16, bind_address: CowArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (u16, CowArg < 'a0, GString >,);
            let args = (port, bind_address,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11292usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UdpServer", "listen", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`listen_ex`][Self::listen_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Starts the server by opening a UDP socket listening on the given `port`. You can optionally specify a `bind_address` to only listen for packets sent to that address. See also [`bind`][`crate::classes::PacketPeerUdp::bind`]."]
        #[inline]
        pub fn listen(&mut self, port: u16,) -> crate::global::Error {
            self.listen_ex(port,) . done()
        }
        #[doc = "Starts the server by opening a UDP socket listening on the given `port`. You can optionally specify a `bind_address` to only listen for packets sent to that address. See also [`bind`][`crate::classes::PacketPeerUdp::bind`]."]
        #[inline]
        pub fn listen_ex < 'ex > (&'ex mut self, port: u16,) -> ExListen < 'ex > {
            ExListen::new(self, port,)
        }
        #[doc = "Call this method at regular intervals (e.g. inside [`process`][`crate::classes::INode::process`]) to process new packets. Any packet from a known address/port pair will be delivered to the appropriate [`PacketPeerUDP`][crate::classes::PacketPeerUdp], while any packet received from an unknown address/port pair will be added as a pending connection (see [`is_connection_available`][`crate::classes::UdpServer::is_connection_available`] and [`take_connection`][`crate::classes::UdpServer::take_connection`]). The maximum number of pending connections is defined via \\[member max_pending_connections]."]
        pub fn poll(&mut self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11293usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UdpServer", "poll", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if a packet with a new address/port combination was received on the socket."]
        pub fn is_connection_available(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11294usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UdpServer", "is_connection_available", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the local port this server is listening to."]
        pub fn get_local_port(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11295usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UdpServer", "get_local_port", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the socket is open and listening on a port."]
        pub fn is_listening(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11296usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UdpServer", "is_listening", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the first pending connection (connected to the appropriate address/port). Will return `null` if no new connection is available. See also [`is_connection_available`][`crate::classes::UdpServer::is_connection_available`], [`connect_to_host`][`crate::classes::PacketPeerUdp::connect_to_host`]."]
        pub fn take_connection(&mut self,) -> Option < Gd < crate::classes::PacketPeerUdp > > {
            type CallRet = Option < Gd < crate::classes::PacketPeerUdp > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11297usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UdpServer", "take_connection", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Stops the server, closing the UDP socket if open. Will close all connected [`PacketPeerUDP`][crate::classes::PacketPeerUdp] accepted via [`take_connection`][`crate::classes::UdpServer::take_connection`] (remote peers will not be notified)."]
        pub fn stop(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11298usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UdpServer", "stop", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_pending_connections(&mut self, max_pending_connections: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (max_pending_connections,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11299usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UdpServer", "set_max_pending_connections", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_pending_connections(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11300usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "UdpServer", "get_max_pending_connections", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for UdpServer {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("UDPServer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for UdpServer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for UdpServer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for UdpServer {
        
    }
    impl crate::obj::cap::GodotDefault for UdpServer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for UdpServer {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for UdpServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`UdpServer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_UdpServer__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::UdpServer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`UdpServer::listen_ex`][super::UdpServer::listen_ex]."]
#[must_use]
pub struct ExListen < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::UdpServer, port: u16, bind_address: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExListen < 'ex > {
    fn new(surround_object: &'ex mut re_export::UdpServer, port: u16,) -> Self {
        let bind_address = GString::from("*");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, port: port, bind_address: CowArg::Owned(bind_address),
        }
    }
    #[inline]
    pub fn bind_address(self, bind_address: impl AsArg < GString > + 'ex) -> Self {
        Self {
            bind_address: bind_address.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, port, bind_address,
        }
        = self;
        re_export::UdpServer::listen_full(surround_object, port, bind_address,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::UdpServer;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for UdpServer {
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
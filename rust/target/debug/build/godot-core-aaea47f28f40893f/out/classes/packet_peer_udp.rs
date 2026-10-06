#![doc = "Sidecar module for class [`PacketPeerUdp`][crate::classes::PacketPeerUdp].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PacketPeerUDP` enums](https://docs.godotengine.org/en/stable/classes/class_packetpeerudp.html#enumerations).\n\n"]
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
    #[doc = "Godot class `PacketPeerUDP`.\n\nInherits [`PacketPeer`][crate::classes::PacketPeer].\n\nRelated symbols:\n\n* [`packet_peer_udp`][crate::classes::packet_peer_udp]: sidecar module with related enum/flag types\n* [`IPacketPeerUdp`][crate::classes::IPacketPeerUdp]: virtual methods\n\n\nSee also [Godot docs for `PacketPeerUDP`](https://docs.godotengine.org/en/stable/classes/class_packetpeerudp.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`PacketPeerUdp::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nUDP packet peer. Can be used to send and receive raw UDP packets as well as [`Variant`][crate::builtin::Variant]s.\n\n**Example:** Send a packet:\n\n```gdscript\nvar peer = PacketPeerUDP.new()\n\n# Optionally, you can select the local port used to send the packet.\npeer.bind(4444)\n\npeer.set_dest_address(\"1.1.1.1\", 4433)\npeer.put_packet(\"hello\".to_utf8_buffer())\n```\n\n**Example:** Listen for packets:\n\n```gdscript\nvar peer\n\nfunc _ready():\n\tpeer = PacketPeerUDP.new()\n\tpeer.bind(4433)\n\n\nfunc _process(_delta):\n\tif peer.get_available_packet_count() > 0:\n\t\tvar array_bytes = peer.get_packet()\n\t\tvar packet_string = array_bytes.get_string_from_ascii()\n\t\tprint(\"Received message: \", packet_string)\n```\n\n**Note:** When exporting to Android, make sure to enable the `INTERNET` permission in the Android export preset before exporting the project or using one-click deploy. Otherwise, network communication of any kind will be blocked by Android."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct PacketPeerUdp {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`PacketPeerUdp`][crate::classes::PacketPeerUdp].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IPacketPeer`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `PacketPeerUDP` methods](https://docs.godotengine.org/en/stable/classes/class_packetpeerudp.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IPacketPeerUdp: crate::obj::GodotClass < Base = PacketPeerUdp > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl PacketPeerUdp {
        #[doc = "Binds this `PacketPeerUDP` to the specified `port` and `bind_address` with a buffer size `recv_buf_size`, allowing it to receive incoming packets.\n\nIf `bind_address` is set to `\"*\"` (default), the peer will be bound on all available addresses (both IPv4 and IPv6).\n\nIf `bind_address` is set to `\"0.0.0.0\"` (for IPv4) or `\"::\"` (for IPv6), the peer will be bound to all available addresses matching that IP type.\n\nIf `bind_address` is set to any valid address (e.g. `\"192.168.1.101\"`, `\"::1\"`, etc.), the peer will only be bound to the interface with that address (or fail if no interface with the given address exists)."]
        pub(crate) fn bind_full(&mut self, port: i32, bind_address: CowArg < GString >, recv_buf_size: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >, i32,);
            let args = (port, bind_address, recv_buf_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11301usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "bind", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`bind_ex`][Self::bind_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Binds this `PacketPeerUDP` to the specified `port` and `bind_address` with a buffer size `recv_buf_size`, allowing it to receive incoming packets.\n\nIf `bind_address` is set to `\"*\"` (default), the peer will be bound on all available addresses (both IPv4 and IPv6).\n\nIf `bind_address` is set to `\"0.0.0.0\"` (for IPv4) or `\"::\"` (for IPv6), the peer will be bound to all available addresses matching that IP type.\n\nIf `bind_address` is set to any valid address (e.g. `\"192.168.1.101\"`, `\"::1\"`, etc.), the peer will only be bound to the interface with that address (or fail if no interface with the given address exists)."]
        #[inline]
        pub fn bind(&mut self, port: i32,) -> crate::global::Error {
            self.bind_ex(port,) . done()
        }
        #[doc = "Binds this `PacketPeerUDP` to the specified `port` and `bind_address` with a buffer size `recv_buf_size`, allowing it to receive incoming packets.\n\nIf `bind_address` is set to `\"*\"` (default), the peer will be bound on all available addresses (both IPv4 and IPv6).\n\nIf `bind_address` is set to `\"0.0.0.0\"` (for IPv4) or `\"::\"` (for IPv6), the peer will be bound to all available addresses matching that IP type.\n\nIf `bind_address` is set to any valid address (e.g. `\"192.168.1.101\"`, `\"::1\"`, etc.), the peer will only be bound to the interface with that address (or fail if no interface with the given address exists)."]
        #[inline]
        pub fn bind_ex < 'ex > (&'ex mut self, port: i32,) -> ExBind < 'ex > {
            ExBind::new(self, port,)
        }
        #[doc = "Closes the `PacketPeerUDP`'s underlying UDP socket."]
        pub fn close(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11302usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "close", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Waits for a packet to arrive on the bound address. See [`bind`][`crate::classes::PacketPeerUdp::bind`].\n\n**Note:** [`wait`][`crate::classes::PacketPeerUdp::wait`] can't be interrupted once it has been called. This can be worked around by allowing the other party to send a specific \"death pill\" packet like this:\n\n\n```gdscript\nsocket = PacketPeerUDP.new()\n# Server\nsocket.set_dest_address(\"127.0.0.1\", 789)\nsocket.put_packet(\"Time to stop\".to_ascii_buffer())\n\n# Client\nwhile socket.wait() == OK:\n\tvar data = socket.get_packet().get_string_from_ascii()\n\tif data == \"Time to stop\":\n\t\treturn\n```\n"]
        pub fn wait(&mut self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11303usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "wait", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns whether this `PacketPeerUDP` is bound to an address and can receive packets."]
        pub fn is_bound(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11304usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "is_bound", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Calling this method connects this UDP peer to the given `host`/`port` pair. UDP is in reality connectionless, so this option only means that incoming packets from different addresses are automatically discarded, and that outgoing packets are always sent to the connected address (future calls to [`set_dest_address`][`crate::classes::PacketPeerUdp::set_dest_address`] are not allowed). This method does not send any data to the remote peer, to do that, use [`put_var`][`crate::classes::PacketPeer::put_var`] or [`put_packet`][`crate::classes::PacketPeer::put_packet`] as usual. See also [`UDPServer`][crate::classes::UdpServer].\n\n**Note:** Connecting to the remote peer does not help to protect from malicious attacks like IP spoofing, etc. Think about using an encryption technique like TLS or DTLS if you feel like your application is transferring sensitive information."]
        pub fn connect_to_host(&mut self, host: impl AsArg < GString >, port: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (host.into_arg(), port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11305usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "connect_to_host", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the UDP socket is open and has been connected to a remote address. See [`connect_to_host`][`crate::classes::PacketPeerUdp::connect_to_host`]."]
        pub fn is_socket_connected(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11306usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "is_socket_connected", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the IP of the remote peer that sent the last packet(that was received with [`get_packet`][`crate::classes::PacketPeer::get_packet`] or [`get_var`][`crate::classes::PacketPeer::get_var`])."]
        pub fn get_packet_ip(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11307usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "get_packet_ip", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the port of the remote peer that sent the last packet(that was received with [`get_packet`][`crate::classes::PacketPeer::get_packet`] or [`get_var`][`crate::classes::PacketPeer::get_var`])."]
        pub fn get_packet_port(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11308usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "get_packet_port", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the local port to which this peer is bound."]
        pub fn get_local_port(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11309usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "get_local_port", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the destination address and port for sending packets and variables. A hostname will be resolved using DNS if needed.\n\n**Note:** [`set_broadcast_enabled`][`crate::classes::PacketPeerUdp::set_broadcast_enabled`] must be enabled before sending packets to a broadcast address (e.g. `255.255.255.255`)."]
        pub fn set_dest_address(&mut self, host: impl AsArg < GString >, port: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (host.into_arg(), port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11310usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "set_dest_address", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Enable or disable sending of broadcast packets (e.g. `set_dest_address(\"255.255.255.255\", 4343)`. This option is disabled by default.\n\n**Note:** Some Android devices might require the `CHANGE_WIFI_MULTICAST_STATE` permission and this option to be enabled to receive broadcast packets too."]
        pub fn set_broadcast_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11311usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "set_broadcast_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Joins the multicast group specified by `multicast_address` using the interface identified by `interface_name`.\n\nYou can join the same multicast group with multiple interfaces. Use [`get_local_interfaces`][`crate::classes::Ip::get_local_interfaces`] to know which are available.\n\n**Note:** Some Android devices might require the `CHANGE_WIFI_MULTICAST_STATE` permission for multicast to work."]
        pub fn join_multicast_group(&mut self, multicast_address: impl AsArg < GString >, interface_name: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (multicast_address.into_arg(), interface_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11312usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "join_multicast_group", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes the interface identified by `interface_name` from the multicast group specified by `multicast_address`."]
        pub fn leave_multicast_group(&mut self, multicast_address: impl AsArg < GString >, interface_name: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (multicast_address.into_arg(), interface_name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11313usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "PacketPeerUdp", "leave_multicast_group", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for PacketPeerUdp {
        type Base = crate::classes::PacketPeer;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("PacketPeerUDP"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for PacketPeerUdp {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PacketPeer > for PacketPeerUdp {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for PacketPeerUdp {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for PacketPeerUdp {
        
    }
    impl crate::obj::cap::GodotDefault for PacketPeerUdp {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for PacketPeerUdp {
        type Target = crate::classes::PacketPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for PacketPeerUdp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`PacketPeerUdp`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_PacketPeerUdp__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::PacketPeerUdp > for $Class {
                
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
#[doc = "Default-param extender for [`PacketPeerUdp::bind_ex`][super::PacketPeerUdp::bind_ex]."]
#[must_use]
pub struct ExBind < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::PacketPeerUdp, port: i32, bind_address: CowArg < 'ex, GString >, recv_buf_size: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBind < 'ex > {
    fn new(surround_object: &'ex mut re_export::PacketPeerUdp, port: i32,) -> Self {
        let bind_address = GString::from("*");
        let recv_buf_size = 65536i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, port: port, bind_address: CowArg::Owned(bind_address), recv_buf_size: recv_buf_size,
        }
    }
    #[inline]
    pub fn bind_address(self, bind_address: impl AsArg < GString > + 'ex) -> Self {
        Self {
            bind_address: bind_address.into_arg(), .. self
        }
    }
    #[inline]
    pub fn recv_buf_size(self, recv_buf_size: i32) -> Self {
        Self {
            recv_buf_size: recv_buf_size, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, port, bind_address, recv_buf_size,
        }
        = self;
        re_export::PacketPeerUdp::bind_full(surround_object, port, bind_address, recv_buf_size,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::PacketPeerUdp;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for PacketPeerUdp {
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
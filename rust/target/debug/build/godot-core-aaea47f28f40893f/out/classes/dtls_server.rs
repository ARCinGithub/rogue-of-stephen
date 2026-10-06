#![doc = "Sidecar module for class [`DtlsServer`][crate::classes::DtlsServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `DTLSServer` enums](https://docs.godotengine.org/en/stable/classes/class_dtlsserver.html#enumerations).\n\n"]
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
    #[doc = "Godot class `DTLSServer`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`IDtlsServer`][crate::classes::IDtlsServer]: virtual methods\n\n\nSee also [Godot docs for `DTLSServer`](https://docs.godotengine.org/en/stable/classes/class_dtlsserver.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`DtlsServer::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nThis class is used to store the state of a DTLS server. Upon [`setup`][`crate::classes::DtlsServer::setup`] it converts connected [`PacketPeerUDP`][crate::classes::PacketPeerUdp] to [`PacketPeerDTLS`][crate::classes::PacketPeerDtls] accepting them via [`take_connection`][`crate::classes::DtlsServer::take_connection`] as DTLS clients. Under the hood, this class is used to store the DTLS state and cookies of the server. The reason of why the state and cookies are needed is outside of the scope of this documentation.\n\nBelow a small example of how to use it:\n\n\n```gdscript\n# server_node.gd\nextends Node\n\nvar dtls = DTLSServer.new()\nvar server = UDPServer.new()\nvar peers = []\n\nfunc _ready():\n\tserver.listen(4242)\n\tvar key = load(\"key.key\") # Your private key.\n\tvar cert = load(\"cert.crt\") # Your X509 certificate.\n\tdtls.setup(TlsOptions.server(key, cert))\n\nfunc _process(delta):\n\twhile server.is_connection_available():\n\t\tvar peer = server.take_connection()\n\t\tvar dtls_peer = dtls.take_connection(peer)\n\t\tif dtls_peer.get_status() != PacketPeerDTLS.STATUS_HANDSHAKING:\n\t\t\tcontinue # It is normal that 50% of the connections fails due to cookie exchange.\n\t\tprint(\"Peer connected!\")\n\t\tpeers.append(dtls_peer)\n\n\tfor p in peers:\n\t\tp.poll() # Must poll to update the state.\n\t\tif p.get_status() == PacketPeerDTLS.STATUS_CONNECTED:\n\t\t\twhile p.get_available_packet_count() > 0:\n\t\t\t\tprint(\"Received message from client: %s\" % p.get_packet().get_string_from_utf8())\n\t\t\t\tp.put_packet(\"Hello DTLS client\".to_utf8_buffer())\n```\n\n\n\n```gdscript\n# client_node.gd\nextends Node\n\nvar dtls = PacketPeerDTLS.new()\nvar udp = PacketPeerUDP.new()\nvar connected = false\n\nfunc _ready():\n\tudp.connect_to_host(\"127.0.0.1\", 4242)\n\tdtls.connect_to_peer(udp, false) # Use true in production for certificate validation!\n\nfunc _process(delta):\n\tdtls.poll()\n\tif dtls.get_status() == PacketPeerDTLS.STATUS_CONNECTED:\n\t\tif !connected:\n\t\t\t# Try to contact server\n\t\t\tdtls.put_packet(\"The answer is... 42!\".to_utf8_buffer())\n\t\twhile dtls.get_available_packet_count() > 0:\n\t\t\tprint(\"Connected: %s\" % dtls.get_packet().get_string_from_utf8())\n\t\t\tconnected = true\n```\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct DtlsServer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`DtlsServer`][crate::classes::DtlsServer].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `DTLSServer` methods](https://docs.godotengine.org/en/stable/classes/class_dtlsserver.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IDtlsServer: crate::obj::GodotClass < Base = DtlsServer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl DtlsServer {
        #[doc = "Setup the DTLS server to use the given `server_options`. See [`server`][`crate::classes::TlsOptions::server`]."]
        pub fn setup(&mut self, server_options: impl AsArg < Option < Gd < crate::classes::TlsOptions >> >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::TlsOptions > > >,);
            let args = (server_options.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11212usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DtlsServer", "setup", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Try to initiate the DTLS handshake with the given `udp_peer` which must be already connected (see [`connect_to_host`][`crate::classes::PacketPeerUdp::connect_to_host`]).\n\n**Note:** You must check that the state of the return PacketPeerUDP is [`Status::HANDSHAKING`][`crate::classes::packet_peer_dtls::Status::HANDSHAKING`], as it is normal that 50% of the new connections will be invalid due to cookie exchange."]
        pub fn take_connection(&mut self, udp_peer: impl AsArg < Option < Gd < crate::classes::PacketPeerUdp >> >,) -> Option < Gd < crate::classes::PacketPeerDtls > > {
            type CallRet = Option < Gd < crate::classes::PacketPeerDtls > >;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::PacketPeerUdp > > >,);
            let args = (udp_peer.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11213usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "DtlsServer", "take_connection", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for DtlsServer {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("DTLSServer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for DtlsServer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for DtlsServer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for DtlsServer {
        
    }
    impl crate::obj::cap::GodotDefault for DtlsServer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for DtlsServer {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for DtlsServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`DtlsServer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_DtlsServer__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::DtlsServer > for $Class {
                
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
    use super::re_export::DtlsServer;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for DtlsServer {
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
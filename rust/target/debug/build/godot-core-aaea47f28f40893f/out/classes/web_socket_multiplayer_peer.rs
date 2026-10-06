#![doc = "Sidecar module for class [`WebSocketMultiplayerPeer`][crate::classes::WebSocketMultiplayerPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `WebSocketMultiplayerPeer` enums](https://docs.godotengine.org/en/stable/classes/class_websocketmultiplayerpeer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `WebSocketMultiplayerPeer`.\n\nInherits [`MultiplayerPeer`][crate::classes::MultiplayerPeer].\n\nRelated symbols:\n\n* [`web_socket_multiplayer_peer`][crate::classes::web_socket_multiplayer_peer]: sidecar module with related enum/flag types\n* [`IWebSocketMultiplayerPeer`][crate::classes::IWebSocketMultiplayerPeer]: virtual methods\n\n\nSee also [Godot docs for `WebSocketMultiplayerPeer`](https://docs.godotengine.org/en/stable/classes/class_websocketmultiplayerpeer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`WebSocketMultiplayerPeer::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nBase class for WebSocket server and client, allowing them to be used as multiplayer peer for the [`MultiplayerAPI`][crate::classes::MultiplayerApi].\n\n**Note:** When exporting to Android, make sure to enable the `INTERNET` permission in the Android export preset before exporting the project or using one-click deploy. Otherwise, network communication of any kind will be blocked by Android."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct WebSocketMultiplayerPeer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`WebSocketMultiplayerPeer`][crate::classes::WebSocketMultiplayerPeer].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IMultiplayerPeer`~~ > ~~`IPacketPeer`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `WebSocketMultiplayerPeer` methods](https://docs.godotengine.org/en/stable/classes/class_websocketmultiplayerpeer.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IWebSocketMultiplayerPeer: crate::obj::GodotClass < Base = WebSocketMultiplayerPeer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl WebSocketMultiplayerPeer {
        #[doc = "Starts a new multiplayer client connecting to the given `url`. TLS certificates will be verified against the hostname when connecting using the `wss://` protocol. You can pass the optional `tls_client_options` parameter to customize the trusted certification authorities, or disable the common name verification. See [`client`][`crate::classes::TlsOptions::client`] and [`client_unsafe`][`crate::classes::TlsOptions::client_unsafe`].\n\n**Note:** It is recommended to specify the scheme part of the URL, i.e. the `url` should start with either `ws://` or `wss://`."]
        pub(crate) fn create_client_full(&mut self, url: CowArg < GString >, tls_client_options: CowArg < Option < Gd < crate::classes::TlsOptions > > >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::TlsOptions > > >,);
            let args = (url, tls_client_options,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7245usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketMultiplayerPeer", "create_client", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_client_ex`][Self::create_client_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Starts a new multiplayer client connecting to the given `url`. TLS certificates will be verified against the hostname when connecting using the `wss://` protocol. You can pass the optional `tls_client_options` parameter to customize the trusted certification authorities, or disable the common name verification. See [`client`][`crate::classes::TlsOptions::client`] and [`client_unsafe`][`crate::classes::TlsOptions::client_unsafe`].\n\n**Note:** It is recommended to specify the scheme part of the URL, i.e. the `url` should start with either `ws://` or `wss://`."]
        #[inline]
        pub fn create_client(&mut self, url: impl AsArg < GString >,) -> crate::global::Error {
            self.create_client_ex(url,) . done()
        }
        #[doc = "Starts a new multiplayer client connecting to the given `url`. TLS certificates will be verified against the hostname when connecting using the `wss://` protocol. You can pass the optional `tls_client_options` parameter to customize the trusted certification authorities, or disable the common name verification. See [`client`][`crate::classes::TlsOptions::client`] and [`client_unsafe`][`crate::classes::TlsOptions::client_unsafe`].\n\n**Note:** It is recommended to specify the scheme part of the URL, i.e. the `url` should start with either `ws://` or `wss://`."]
        #[inline]
        pub fn create_client_ex < 'ex > (&'ex mut self, url: impl AsArg < GString > + 'ex,) -> ExCreateClient < 'ex > {
            ExCreateClient::new(self, url,)
        }
        #[doc = "Starts a new multiplayer server listening on the given `port`. You can optionally specify a `bind_address`, and provide valid `tls_server_options` to use TLS. See [`server`][`crate::classes::TlsOptions::server`]."]
        pub(crate) fn create_server_full(&mut self, port: i32, bind_address: CowArg < GString >, tls_server_options: CowArg < Option < Gd < crate::classes::TlsOptions > > >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (i32, CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::TlsOptions > > >,);
            let args = (port, bind_address, tls_server_options,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7246usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketMultiplayerPeer", "create_server", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_server_ex`][Self::create_server_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Starts a new multiplayer server listening on the given `port`. You can optionally specify a `bind_address`, and provide valid `tls_server_options` to use TLS. See [`server`][`crate::classes::TlsOptions::server`]."]
        #[inline]
        pub fn create_server(&mut self, port: i32,) -> crate::global::Error {
            self.create_server_ex(port,) . done()
        }
        #[doc = "Starts a new multiplayer server listening on the given `port`. You can optionally specify a `bind_address`, and provide valid `tls_server_options` to use TLS. See [`server`][`crate::classes::TlsOptions::server`]."]
        #[inline]
        pub fn create_server_ex < 'ex > (&'ex mut self, port: i32,) -> ExCreateServer < 'ex > {
            ExCreateServer::new(self, port,)
        }
        #[doc = "Returns the [`WebSocketPeer`][crate::classes::WebSocketPeer] associated to the given `peer_id`."]
        pub fn get_peer(&self, peer_id: i32,) -> Option < Gd < crate::classes::WebSocketPeer > > {
            type CallRet = Option < Gd < crate::classes::WebSocketPeer > >;
            type CallParams = (i32,);
            let args = (peer_id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7247usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketMultiplayerPeer", "get_peer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the IP address of the given peer."]
        pub fn get_peer_address(&self, id: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7248usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketMultiplayerPeer", "get_peer_address", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the remote port of the given peer."]
        pub fn get_peer_port(&self, id: i32,) -> i32 {
            type CallRet = i32;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7249usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketMultiplayerPeer", "get_peer_port", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_supported_protocols(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7250usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketMultiplayerPeer", "get_supported_protocols", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_supported_protocols(&mut self, protocols: &PackedStringArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
            let args = (RefArg::new(protocols),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7251usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketMultiplayerPeer", "set_supported_protocols", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_handshake_headers(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7252usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketMultiplayerPeer", "get_handshake_headers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_handshake_headers(&mut self, protocols: &PackedStringArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
            let args = (RefArg::new(protocols),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7253usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketMultiplayerPeer", "set_handshake_headers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_inbound_buffer_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7254usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketMultiplayerPeer", "get_inbound_buffer_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_inbound_buffer_size(&mut self, buffer_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (buffer_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7255usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketMultiplayerPeer", "set_inbound_buffer_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_outbound_buffer_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7256usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketMultiplayerPeer", "get_outbound_buffer_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_outbound_buffer_size(&mut self, buffer_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (buffer_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7257usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketMultiplayerPeer", "set_outbound_buffer_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_handshake_timeout(&self,) -> f32 {
            type CallRet = f32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7258usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketMultiplayerPeer", "get_handshake_timeout", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_handshake_timeout(&mut self, timeout: f32,) {
            type CallRet = ();
            type CallParams = (f32,);
            let args = (timeout,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7259usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketMultiplayerPeer", "set_handshake_timeout", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_queued_packets(&mut self, max_queued_packets: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (max_queued_packets,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7260usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketMultiplayerPeer", "set_max_queued_packets", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_queued_packets(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7261usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketMultiplayerPeer", "get_max_queued_packets", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for WebSocketMultiplayerPeer {
        type Base = crate::classes::MultiplayerPeer;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("WebSocketMultiplayerPeer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for WebSocketMultiplayerPeer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::MultiplayerPeer > for WebSocketMultiplayerPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PacketPeer > for WebSocketMultiplayerPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for WebSocketMultiplayerPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for WebSocketMultiplayerPeer {
        
    }
    impl crate::obj::cap::GodotDefault for WebSocketMultiplayerPeer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for WebSocketMultiplayerPeer {
        type Target = crate::classes::MultiplayerPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for WebSocketMultiplayerPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`WebSocketMultiplayerPeer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_WebSocketMultiplayerPeer__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::WebSocketMultiplayerPeer > for $Class {
                
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
#[doc = "Default-param extender for [`WebSocketMultiplayerPeer::create_client_ex`][super::WebSocketMultiplayerPeer::create_client_ex]."]
#[must_use]
pub struct ExCreateClient < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::WebSocketMultiplayerPeer, url: CowArg < 'ex, GString >, tls_client_options: CowArg < 'ex, Option < Gd < crate::classes::TlsOptions > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateClient < 'ex > {
    fn new(surround_object: &'ex mut re_export::WebSocketMultiplayerPeer, url: impl AsArg < GString > + 'ex,) -> Self {
        let tls_client_options = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, url: url.into_arg(), tls_client_options: tls_client_options.into_arg(),
        }
    }
    #[inline]
    pub fn tls_client_options(self, tls_client_options: impl AsArg < Option < Gd < crate::classes::TlsOptions >> > + 'ex) -> Self {
        Self {
            tls_client_options: tls_client_options.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, url, tls_client_options,
        }
        = self;
        re_export::WebSocketMultiplayerPeer::create_client_full(surround_object, url, tls_client_options,)
    }
}
#[doc = "Default-param extender for [`WebSocketMultiplayerPeer::create_server_ex`][super::WebSocketMultiplayerPeer::create_server_ex]."]
#[must_use]
pub struct ExCreateServer < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::WebSocketMultiplayerPeer, port: i32, bind_address: CowArg < 'ex, GString >, tls_server_options: CowArg < 'ex, Option < Gd < crate::classes::TlsOptions > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateServer < 'ex > {
    fn new(surround_object: &'ex mut re_export::WebSocketMultiplayerPeer, port: i32,) -> Self {
        let bind_address = GString::from("*");
        let tls_server_options = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, port: port, bind_address: CowArg::Owned(bind_address), tls_server_options: tls_server_options.into_arg(),
        }
    }
    #[inline]
    pub fn bind_address(self, bind_address: impl AsArg < GString > + 'ex) -> Self {
        Self {
            bind_address: bind_address.into_arg(), .. self
        }
    }
    #[inline]
    pub fn tls_server_options(self, tls_server_options: impl AsArg < Option < Gd < crate::classes::TlsOptions >> > + 'ex) -> Self {
        Self {
            tls_server_options: tls_server_options.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, port, bind_address, tls_server_options,
        }
        = self;
        re_export::WebSocketMultiplayerPeer::create_server_full(surround_object, port, bind_address, tls_server_options,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::WebSocketMultiplayerPeer;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::multiplayer_peer::SignalsOfMultiplayerPeer;
    impl WithSignals for WebSocketMultiplayerPeer {
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
#![doc = "Sidecar module for class [`WebSocketPeer`][crate::classes::WebSocketPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `WebSocketPeer` enums](https://docs.godotengine.org/en/stable/classes/class_websocketpeer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `WebSocketPeer`.\n\nInherits [`PacketPeer`][crate::classes::PacketPeer].\n\nRelated symbols:\n\n* [`web_socket_peer`][crate::classes::web_socket_peer]: sidecar module with related enum/flag types\n* [`IWebSocketPeer`][crate::classes::IWebSocketPeer]: virtual methods\n\n\nSee also [Godot docs for `WebSocketPeer`](https://docs.godotengine.org/en/stable/classes/class_websocketpeer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`WebSocketPeer::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nThis class represents WebSocket connection, and can be used as a WebSocket client ([RFC 6455](https://datatracker.ietf.org/doc/html/rfc6455)-compliant) or as a remote peer of a WebSocket server.\n\nYou can send WebSocket binary frames using [`put_packet`][`crate::classes::PacketPeer::put_packet`], and WebSocket text frames using [`send`][`crate::classes::WebSocketPeer::send`] (prefer text frames when interacting with text-based API). You can check the frame type of the last packet via [`was_string_packet`][`crate::classes::WebSocketPeer::was_string_packet`].\n\nTo start a WebSocket client, first call [`connect_to_url`][`crate::classes::WebSocketPeer::connect_to_url`], then regularly call [`poll`][`crate::classes::WebSocketPeer::poll`] (e.g. during [`Node`][crate::classes::Node] process). You can query the socket state via [`get_ready_state`][`crate::classes::WebSocketPeer::get_ready_state`], get the number of pending packets using [`get_available_packet_count`][`crate::classes::PacketPeer::get_available_packet_count`], and retrieve them via [`get_packet`][`crate::classes::PacketPeer::get_packet`].\n\n\n```gdscript\nextends Node\n\nvar socket = WebSocketPeer.new()\n\nfunc _ready():\n\tsocket.connect_to_url(\"wss://example.com\")\n\nfunc _process(delta):\n\tsocket.poll()\n\tvar state = socket.get_ready_state()\n\tif state == WebSocketPeer.STATE_OPEN:\n\t\twhile socket.get_available_packet_count():\n\t\t\tprint(\"Packet: \", socket.get_packet())\n\telif state == WebSocketPeer.STATE_CLOSING:\n\t\t# Keep polling to achieve proper close.\n\t\tpass\n\telif state == WebSocketPeer.STATE_CLOSED:\n\t\tvar code = socket.get_close_code()\n\t\tvar reason = socket.get_close_reason()\n\t\tprint(\"WebSocket closed with code: %d, reason %s. Clean: %s\" % [code, reason, code != -1])\n\t\tset_process(false) # Stop processing.\n```\n\n\nTo use the peer as part of a WebSocket server refer to [`accept_stream`][`crate::classes::WebSocketPeer::accept_stream`] and the online tutorial."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct WebSocketPeer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`WebSocketPeer`][crate::classes::WebSocketPeer].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IPacketPeer`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `WebSocketPeer` methods](https://docs.godotengine.org/en/stable/classes/class_websocketpeer.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IWebSocketPeer: crate::obj::GodotClass < Base = WebSocketPeer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl WebSocketPeer {
        #[doc = "Connects to the given URL. TLS certificates will be verified against the hostname when connecting using the `wss://` protocol. You can pass the optional `tls_client_options` parameter to customize the trusted certification authorities, or disable the common name verification. See [`client`][`crate::classes::TlsOptions::client`] and [`client_unsafe`][`crate::classes::TlsOptions::client_unsafe`].\n\n**Note:** This method is non-blocking, and will return [`Error::OK`][`crate::global::Error::OK`] before the connection is established as long as the provided parameters are valid and the peer is not in an invalid state (e.g. already connected). Regularly call [`poll`][`crate::classes::WebSocketPeer::poll`] (e.g. during [`Node`][crate::classes::Node] process) and check the result of [`get_ready_state`][`crate::classes::WebSocketPeer::get_ready_state`] to know whether the connection succeeds or fails.\n\n**Note:** To avoid mixed content warnings or errors in Web, you may have to use a `url` that starts with `wss://` (secure) instead of `ws://`. When doing so, make sure to use the fully qualified domain name that matches the one defined in the server's TLS certificate. Do not connect directly via the IP address for `wss://` connections, as it won't match with the TLS certificate."]
        pub(crate) fn connect_to_url_full(&mut self, url: CowArg < GString >, tls_client_options: CowArg < Option < Gd < crate::classes::TlsOptions > > >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, Option < Gd < crate::classes::TlsOptions > > >,);
            let args = (url, tls_client_options,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7217usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "connect_to_url", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`connect_to_url_ex`][Self::connect_to_url_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Connects to the given URL. TLS certificates will be verified against the hostname when connecting using the `wss://` protocol. You can pass the optional `tls_client_options` parameter to customize the trusted certification authorities, or disable the common name verification. See [`client`][`crate::classes::TlsOptions::client`] and [`client_unsafe`][`crate::classes::TlsOptions::client_unsafe`].\n\n**Note:** This method is non-blocking, and will return [`Error::OK`][`crate::global::Error::OK`] before the connection is established as long as the provided parameters are valid and the peer is not in an invalid state (e.g. already connected). Regularly call [`poll`][`crate::classes::WebSocketPeer::poll`] (e.g. during [`Node`][crate::classes::Node] process) and check the result of [`get_ready_state`][`crate::classes::WebSocketPeer::get_ready_state`] to know whether the connection succeeds or fails.\n\n**Note:** To avoid mixed content warnings or errors in Web, you may have to use a `url` that starts with `wss://` (secure) instead of `ws://`. When doing so, make sure to use the fully qualified domain name that matches the one defined in the server's TLS certificate. Do not connect directly via the IP address for `wss://` connections, as it won't match with the TLS certificate."]
        #[inline]
        pub fn connect_to_url(&mut self, url: impl AsArg < GString >,) -> crate::global::Error {
            self.connect_to_url_ex(url,) . done()
        }
        #[doc = "Connects to the given URL. TLS certificates will be verified against the hostname when connecting using the `wss://` protocol. You can pass the optional `tls_client_options` parameter to customize the trusted certification authorities, or disable the common name verification. See [`client`][`crate::classes::TlsOptions::client`] and [`client_unsafe`][`crate::classes::TlsOptions::client_unsafe`].\n\n**Note:** This method is non-blocking, and will return [`Error::OK`][`crate::global::Error::OK`] before the connection is established as long as the provided parameters are valid and the peer is not in an invalid state (e.g. already connected). Regularly call [`poll`][`crate::classes::WebSocketPeer::poll`] (e.g. during [`Node`][crate::classes::Node] process) and check the result of [`get_ready_state`][`crate::classes::WebSocketPeer::get_ready_state`] to know whether the connection succeeds or fails.\n\n**Note:** To avoid mixed content warnings or errors in Web, you may have to use a `url` that starts with `wss://` (secure) instead of `ws://`. When doing so, make sure to use the fully qualified domain name that matches the one defined in the server's TLS certificate. Do not connect directly via the IP address for `wss://` connections, as it won't match with the TLS certificate."]
        #[inline]
        pub fn connect_to_url_ex < 'ex > (&'ex mut self, url: impl AsArg < GString > + 'ex,) -> ExConnectToUrl < 'ex > {
            ExConnectToUrl::new(self, url,)
        }
        #[doc = "Accepts a peer connection performing the HTTP handshake as a WebSocket server. The `stream` must be a valid TCP stream retrieved via [`take_connection`][`crate::classes::TcpServer::take_connection`], or a TLS stream accepted via [`accept_stream`][`crate::classes::StreamPeerTls::accept_stream`].\n\n**Note:** Not supported in Web exports due to browsers' restrictions."]
        pub fn accept_stream(&mut self, stream: impl AsArg < Option < Gd < crate::classes::StreamPeer >> >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::StreamPeer > > >,);
            let args = (stream.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7218usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "accept_stream", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sends the given `message` using the desired `write_mode`. When sending a [`String`][crate::builtin::GString], prefer using [`send_text`][`crate::classes::WebSocketPeer::send_text`]."]
        pub(crate) fn send_full(&mut self, message: RefArg < PackedByteArray >, write_mode: crate::classes::web_socket_peer::WriteMode,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >, crate::classes::web_socket_peer::WriteMode,);
            let args = (message, write_mode,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7219usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "send", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`send_ex`][Self::send_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sends the given `message` using the desired `write_mode`. When sending a [`String`][crate::builtin::GString], prefer using [`send_text`][`crate::classes::WebSocketPeer::send_text`]."]
        #[inline]
        pub fn send(&mut self, message: &PackedByteArray,) -> crate::global::Error {
            self.send_ex(message,) . done()
        }
        #[doc = "Sends the given `message` using the desired `write_mode`. When sending a [`String`][crate::builtin::GString], prefer using [`send_text`][`crate::classes::WebSocketPeer::send_text`]."]
        #[inline]
        pub fn send_ex < 'ex > (&'ex mut self, message: &'ex PackedByteArray,) -> ExSend < 'ex > {
            ExSend::new(self, message,)
        }
        #[doc = "Sends the given `message` using WebSocket text mode. Prefer this method over [`put_packet`][`crate::classes::PacketPeer::put_packet`] when interacting with third-party text-based API (e.g. when using [`JSON`][crate::classes::Json] formatted messages)."]
        pub fn send_text(&mut self, message: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (message.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7220usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "send_text", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if the last received packet was sent as a text payload. See \\[enum WriteMode]."]
        pub fn was_string_packet(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7221usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "was_string_packet", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Updates the connection state and receive incoming packets. Call this function regularly to keep it in a clean state."]
        pub fn poll(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7222usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "poll", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Closes this WebSocket connection.\n\n`code` is the status code for the closure (see [RFC 6455 section 7.4](https://datatracker.ietf.org/doc/html/rfc6455#section-7.4.1) for a list of valid status codes). If `code` is negative, the connection will be closed immediately without notifying the remote peer.\n\n`reason` is the human-readable reason for closing the connection. It can be any UTF-8 string that's smaller than 123 bytes.\n\n**Note:** To achieve a clean closure, you will need to keep polling until [`State::CLOSED`][`crate::classes::web_socket_peer::State::CLOSED`] is reached.\n\n**Note:** The Web export might not support all status codes. Please refer to browser-specific documentation for more details."]
        pub(crate) fn close_full(&mut self, code: i32, reason: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (code, reason,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7223usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "close", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`close_ex`][Self::close_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Closes this WebSocket connection.\n\n`code` is the status code for the closure (see [RFC 6455 section 7.4](https://datatracker.ietf.org/doc/html/rfc6455#section-7.4.1) for a list of valid status codes). If `code` is negative, the connection will be closed immediately without notifying the remote peer.\n\n`reason` is the human-readable reason for closing the connection. It can be any UTF-8 string that's smaller than 123 bytes.\n\n**Note:** To achieve a clean closure, you will need to keep polling until [`State::CLOSED`][`crate::classes::web_socket_peer::State::CLOSED`] is reached.\n\n**Note:** The Web export might not support all status codes. Please refer to browser-specific documentation for more details."]
        #[inline]
        pub fn close(&mut self,) {
            self.close_ex() . done()
        }
        #[doc = "Closes this WebSocket connection.\n\n`code` is the status code for the closure (see [RFC 6455 section 7.4](https://datatracker.ietf.org/doc/html/rfc6455#section-7.4.1) for a list of valid status codes). If `code` is negative, the connection will be closed immediately without notifying the remote peer.\n\n`reason` is the human-readable reason for closing the connection. It can be any UTF-8 string that's smaller than 123 bytes.\n\n**Note:** To achieve a clean closure, you will need to keep polling until [`State::CLOSED`][`crate::classes::web_socket_peer::State::CLOSED`] is reached.\n\n**Note:** The Web export might not support all status codes. Please refer to browser-specific documentation for more details."]
        #[inline]
        pub fn close_ex < 'ex > (&'ex mut self,) -> ExClose < 'ex > {
            ExClose::new(self,)
        }
        #[doc = "Returns the IP address of the connected peer.\n\n**Note:** Not available in the Web export."]
        pub fn get_connected_host(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7224usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_connected_host", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the remote port of the connected peer.\n\n**Note:** Not available in the Web export."]
        pub fn get_connected_port(&self,) -> u16 {
            type CallRet = u16;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7225usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_connected_port", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the selected WebSocket sub-protocol for this connection or an empty string if the sub-protocol has not been selected yet."]
        pub fn get_selected_protocol(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7226usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_selected_protocol", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the URL requested by this peer. The URL is derived from the `url` passed to [`connect_to_url`][`crate::classes::WebSocketPeer::connect_to_url`] or from the HTTP headers when acting as server (i.e. when using [`accept_stream`][`crate::classes::WebSocketPeer::accept_stream`])."]
        pub fn get_requested_url(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7227usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_requested_url", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Disable Nagle's algorithm on the underlying TCP socket (default). See [`set_no_delay`][`crate::classes::StreamPeerTcp::set_no_delay`] for more information.\n\n**Note:** Not available in the Web export."]
        pub fn set_no_delay(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7228usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "set_no_delay", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the current amount of data in the outbound websocket buffer. **Note:** Web exports use WebSocket.bufferedAmount, while other platforms use an internal buffer."]
        pub fn get_current_outbound_buffered_amount(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7229usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_current_outbound_buffered_amount", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the ready state of the connection."]
        pub fn get_ready_state(&self,) -> crate::classes::web_socket_peer::State {
            type CallRet = crate::classes::web_socket_peer::State;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7230usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_ready_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the received WebSocket close frame status code, or `-1` when the connection was not cleanly closed. Only call this method when [`get_ready_state`][`crate::classes::WebSocketPeer::get_ready_state`] returns [`State::CLOSED`][`crate::classes::web_socket_peer::State::CLOSED`]."]
        pub fn get_close_code(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7231usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_close_code", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the received WebSocket close frame status reason string. Only call this method when [`get_ready_state`][`crate::classes::WebSocketPeer::get_ready_state`] returns [`State::CLOSED`][`crate::classes::web_socket_peer::State::CLOSED`]."]
        pub fn get_close_reason(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7232usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_close_reason", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_supported_protocols(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7233usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_supported_protocols", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_supported_protocols(&mut self, protocols: &PackedStringArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
            let args = (RefArg::new(protocols),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7234usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "set_supported_protocols", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_handshake_headers(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7235usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_handshake_headers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_handshake_headers(&mut self, protocols: &PackedStringArray,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, PackedStringArray >,);
            let args = (RefArg::new(protocols),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "set_handshake_headers", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_inbound_buffer_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_inbound_buffer_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_inbound_buffer_size(&mut self, buffer_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (buffer_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "set_inbound_buffer_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_outbound_buffer_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_outbound_buffer_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_outbound_buffer_size(&mut self, buffer_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (buffer_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "set_outbound_buffer_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_queued_packets(&mut self, buffer_size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (buffer_size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "set_max_queued_packets", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_queued_packets(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_max_queued_packets", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_heartbeat_interval(&mut self, interval: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (interval,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "set_heartbeat_interval", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_heartbeat_interval(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebSocketPeer", "get_heartbeat_interval", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for WebSocketPeer {
        type Base = crate::classes::PacketPeer;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("WebSocketPeer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for WebSocketPeer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PacketPeer > for WebSocketPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for WebSocketPeer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for WebSocketPeer {
        
    }
    impl crate::obj::cap::GodotDefault for WebSocketPeer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for WebSocketPeer {
        type Target = crate::classes::PacketPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for WebSocketPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`WebSocketPeer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_WebSocketPeer__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::WebSocketPeer > for $Class {
                
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
#[doc = "Default-param extender for [`WebSocketPeer::connect_to_url_ex`][super::WebSocketPeer::connect_to_url_ex]."]
#[must_use]
pub struct ExConnectToUrl < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::WebSocketPeer, url: CowArg < 'ex, GString >, tls_client_options: CowArg < 'ex, Option < Gd < crate::classes::TlsOptions > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExConnectToUrl < 'ex > {
    fn new(surround_object: &'ex mut re_export::WebSocketPeer, url: impl AsArg < GString > + 'ex,) -> Self {
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
        re_export::WebSocketPeer::connect_to_url_full(surround_object, url, tls_client_options,)
    }
}
#[doc = "Default-param extender for [`WebSocketPeer::send_ex`][super::WebSocketPeer::send_ex]."]
#[must_use]
pub struct ExSend < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::WebSocketPeer, message: CowArg < 'ex, PackedByteArray >, write_mode: crate::classes::web_socket_peer::WriteMode,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSend < 'ex > {
    fn new(surround_object: &'ex mut re_export::WebSocketPeer, message: &'ex PackedByteArray,) -> Self {
        let write_mode = crate::obj::EngineEnum::from_ord(1);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, message: CowArg::Borrowed(message), write_mode: write_mode,
        }
    }
    #[inline]
    pub fn write_mode(self, write_mode: crate::classes::web_socket_peer::WriteMode) -> Self {
        Self {
            write_mode: write_mode, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, message, write_mode,
        }
        = self;
        re_export::WebSocketPeer::send_full(surround_object, message.cow_as_arg(), write_mode,)
    }
}
#[doc = "Default-param extender for [`WebSocketPeer::close_ex`][super::WebSocketPeer::close_ex]."]
#[must_use]
pub struct ExClose < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::WebSocketPeer, code: i32, reason: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExClose < 'ex > {
    fn new(surround_object: &'ex mut re_export::WebSocketPeer,) -> Self {
        let code = 1000i32;
        let reason = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, code: code, reason: CowArg::Owned(reason),
        }
    }
    #[inline]
    pub fn code(self, code: i32) -> Self {
        Self {
            code: code, .. self
        }
    }
    #[inline]
    pub fn reason(self, reason: impl AsArg < GString > + 'ex) -> Self {
        Self {
            reason: reason.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, code, reason,
        }
        = self;
        re_export::WebSocketPeer::close_full(surround_object, code, reason,)
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
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("WebSocketPeer.WriteMode")), is_bitfield: false,
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
pub struct State {
    ord: i32
}
impl State {
    #[doc(alias = "STATE_CONNECTING")]
    #[doc = "Godot enumerator name: `STATE_CONNECTING`"]
    pub const CONNECTING: State = State {
        ord: 0i32
    };
    #[doc(alias = "STATE_OPEN")]
    #[doc = "Godot enumerator name: `STATE_OPEN`"]
    pub const OPEN: State = State {
        ord: 1i32
    };
    #[doc(alias = "STATE_CLOSING")]
    #[doc = "Godot enumerator name: `STATE_CLOSING`"]
    pub const CLOSING: State = State {
        ord: 2i32
    };
    #[doc(alias = "STATE_CLOSED")]
    #[doc = "Godot enumerator name: `STATE_CLOSED`"]
    pub const CLOSED: State = State {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("State") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for State {
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
        &[State::CONNECTING, State::OPEN, State::CLOSING, State::CLOSED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < State >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("CONNECTING", "STATE_CONNECTING", State::CONNECTING), crate::meta::inspect::EnumConstant::new("OPEN", "STATE_OPEN", State::OPEN), crate::meta::inspect::EnumConstant::new("CLOSING", "STATE_CLOSING", State::CLOSING), crate::meta::inspect::EnumConstant::new("CLOSED", "STATE_CLOSED", State::CLOSED)]
        }
    }
}
impl crate::meta::GodotConvert for State {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("State Connecting", 0i64), EnumeratorShape::new_int("State Open", 1i64), EnumeratorShape::new_int("State Closing", 2i64), EnumeratorShape::new_int("State Closed", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("WebSocketPeer.State")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for State {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for State {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for State {
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
impl crate::registry::property::Export for State {
    
}
impl crate::meta::Element for State {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::WebSocketPeer;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for WebSocketPeer {
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
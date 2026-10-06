#![doc = "Sidecar module for class [`WebRtcPeerConnection`][crate::classes::WebRtcPeerConnection].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `WebRTCPeerConnection` enums](https://docs.godotengine.org/en/stable/classes/class_webrtcpeerconnection.html#enumerations).\n\n"]
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
    #[doc = "Godot class `WebRTCPeerConnection`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`web_rtc_peer_connection`][crate::classes::web_rtc_peer_connection]: sidecar module with related enum/flag types\n* [`IWebRtcPeerConnection`][crate::classes::IWebRtcPeerConnection]: virtual methods\n* [`SignalsOfWebRtcPeerConnection`][crate::classes::web_rtc_peer_connection::SignalsOfWebRtcPeerConnection]: signal collection\n\n\nSee also [Godot docs for `WebRTCPeerConnection`](https://docs.godotengine.org/en/stable/classes/class_webrtcpeerconnection.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`WebRtcPeerConnection::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nA WebRTC connection between the local computer and a remote peer. Provides an interface to connect, maintain, and monitor the connection.\n\nSetting up a WebRTC connection between two peers may not seem a trivial task, but it can be broken down into 3 main steps:\n\n- The peer that wants to initiate the connection (`A` from now on) creates an offer and sends it to the other peer (`B` from now on).\n\n- `B` receives the offer, generates an answer, and sends it to `A`.\n\n- `A` and `B` then generate and exchange ICE candidates with each other.\n\nAfter these steps, the connection should be established. Refer to the linked tutorials for details."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct WebRtcPeerConnection {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`WebRtcPeerConnection`][crate::classes::WebRtcPeerConnection].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].\n\nSee also [Godot docs for `WebRTCPeerConnection` methods](https://docs.godotengine.org/en/stable/classes/class_webrtcpeerconnection.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IWebRtcPeerConnection: crate::obj::GodotClass < Base = WebRtcPeerConnection > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl WebRtcPeerConnection {
        #[doc = "Sets the `extension_class` as the default [`WebRTCPeerConnectionExtension`][crate::classes::WebRtcPeerConnectionExtension] returned when creating a new `WebRTCPeerConnection`."]
        pub fn set_default_extension(extension_class: impl AsArg < StringName >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, StringName >,);
            let args = (extension_class.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7262usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "set_default_extension", None, args,)
            }
        }
        #[doc = "Re-initialize this peer connection, closing any previously active connection, and going back to state [`ConnectionState::NEW`][`crate::classes::web_rtc_peer_connection::ConnectionState::NEW`]. A dictionary of `configuration` options can be passed to configure the peer connection.\n\nValid `configuration` options are:\n\n```gdscript\n{\n\t\"iceServers\": [\n\t\t{\n\t\t\t\"urls\": [ \"stun:stun.example.com:3478\" ], # One or more STUN servers.\n\t\t},\n\t\t{\n\t\t\t\"urls\": [ \"turn:turn.example.com:3478\" ], # One or more TURN servers.\n\t\t\t\"username\": \"a_username\", # Optional username for the TURN server.\n\t\t\t\"credential\": \"a_password\", # Optional password for the TURN server.\n\t\t}\n\t]\n}\n```"]
        pub(crate) fn initialize_full(&mut self, configuration: RefArg < AnyDictionary >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, AnyDictionary >,);
            let args = (configuration,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7263usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "initialize", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`initialize_ex`][Self::initialize_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Re-initialize this peer connection, closing any previously active connection, and going back to state [`ConnectionState::NEW`][`crate::classes::web_rtc_peer_connection::ConnectionState::NEW`]. A dictionary of `configuration` options can be passed to configure the peer connection.\n\nValid `configuration` options are:\n\n```gdscript\n{\n\t\"iceServers\": [\n\t\t{\n\t\t\t\"urls\": [ \"stun:stun.example.com:3478\" ], # One or more STUN servers.\n\t\t},\n\t\t{\n\t\t\t\"urls\": [ \"turn:turn.example.com:3478\" ], # One or more TURN servers.\n\t\t\t\"username\": \"a_username\", # Optional username for the TURN server.\n\t\t\t\"credential\": \"a_password\", # Optional password for the TURN server.\n\t\t}\n\t]\n}\n```"]
        #[inline]
        pub fn initialize(&mut self,) -> crate::global::Error {
            self.initialize_ex() . done()
        }
        #[doc = "Re-initialize this peer connection, closing any previously active connection, and going back to state [`ConnectionState::NEW`][`crate::classes::web_rtc_peer_connection::ConnectionState::NEW`]. A dictionary of `configuration` options can be passed to configure the peer connection.\n\nValid `configuration` options are:\n\n```gdscript\n{\n\t\"iceServers\": [\n\t\t{\n\t\t\t\"urls\": [ \"stun:stun.example.com:3478\" ], # One or more STUN servers.\n\t\t},\n\t\t{\n\t\t\t\"urls\": [ \"turn:turn.example.com:3478\" ], # One or more TURN servers.\n\t\t\t\"username\": \"a_username\", # Optional username for the TURN server.\n\t\t\t\"credential\": \"a_password\", # Optional password for the TURN server.\n\t\t}\n\t]\n}\n```"]
        #[inline]
        pub fn initialize_ex < 'ex > (&'ex mut self,) -> ExInitialize < 'ex > {
            ExInitialize::new(self,)
        }
        #[doc = "Returns a new [`WebRTCDataChannel`][crate::classes::WebRtcDataChannel] (or `null` on failure) with given `label` and optionally configured via the `options` dictionary. This method can only be called when the connection is in state [`ConnectionState::NEW`][`crate::classes::web_rtc_peer_connection::ConnectionState::NEW`].\n\nThere are two ways to create a working data channel: either call [`create_data_channel`][`crate::classes::WebRtcPeerConnection::create_data_channel`] on only one of the peer and listen to `data_channel_received` on the other, or call [`create_data_channel`][`crate::classes::WebRtcPeerConnection::create_data_channel`] on both peers, with the same values, and the `\"negotiated\"` option set to `true`.\n\nValid `options` are:\n\n```gdscript\n{\n\t\"negotiated\": true, # When set to true (default off), means the channel is negotiated out of band. \"id\" must be set too. \"data_channel_received\" will not be called.\n\t\"id\": 1, # When \"negotiated\" is true this value must also be set to the same value on both peer.\n\n\t# Only one of maxRetransmits and maxPacketLifeTime can be specified, not both. They make the channel unreliable (but also better at real time).\n\t\"maxRetransmits\": 1, # Specify the maximum number of attempt the peer will make to retransmits packets if they are not acknowledged.\n\t\"maxPacketLifeTime\": 100, # Specify the maximum amount of time before giving up retransmitions of unacknowledged packets (in milliseconds).\n\t\"ordered\": true, # When in unreliable mode (i.e. either \"maxRetransmits\" or \"maxPacketLifetime\" is set), \"ordered\" (true by default) specify if packet ordering is to be enforced.\n\n\t\"protocol\": \"my-custom-protocol\", # A custom sub-protocol string for this channel.\n}\n```\n\n**Note:** You must keep a reference to channels created this way, or it will be closed."]
        pub(crate) fn create_data_channel_full(&mut self, label: CowArg < GString >, options: RefArg < AnyDictionary >,) -> Option < Gd < crate::classes::WebRtcDataChannel > > {
            type CallRet = Option < Gd < crate::classes::WebRtcDataChannel > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, RefArg < 'a1, AnyDictionary >,);
            let args = (label, options,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7264usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "create_data_channel", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`create_data_channel_ex`][Self::create_data_channel_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a new [`WebRTCDataChannel`][crate::classes::WebRtcDataChannel] (or `null` on failure) with given `label` and optionally configured via the `options` dictionary. This method can only be called when the connection is in state [`ConnectionState::NEW`][`crate::classes::web_rtc_peer_connection::ConnectionState::NEW`].\n\nThere are two ways to create a working data channel: either call [`create_data_channel`][`crate::classes::WebRtcPeerConnection::create_data_channel`] on only one of the peer and listen to `data_channel_received` on the other, or call [`create_data_channel`][`crate::classes::WebRtcPeerConnection::create_data_channel`] on both peers, with the same values, and the `\"negotiated\"` option set to `true`.\n\nValid `options` are:\n\n```gdscript\n{\n\t\"negotiated\": true, # When set to true (default off), means the channel is negotiated out of band. \"id\" must be set too. \"data_channel_received\" will not be called.\n\t\"id\": 1, # When \"negotiated\" is true this value must also be set to the same value on both peer.\n\n\t# Only one of maxRetransmits and maxPacketLifeTime can be specified, not both. They make the channel unreliable (but also better at real time).\n\t\"maxRetransmits\": 1, # Specify the maximum number of attempt the peer will make to retransmits packets if they are not acknowledged.\n\t\"maxPacketLifeTime\": 100, # Specify the maximum amount of time before giving up retransmitions of unacknowledged packets (in milliseconds).\n\t\"ordered\": true, # When in unreliable mode (i.e. either \"maxRetransmits\" or \"maxPacketLifetime\" is set), \"ordered\" (true by default) specify if packet ordering is to be enforced.\n\n\t\"protocol\": \"my-custom-protocol\", # A custom sub-protocol string for this channel.\n}\n```\n\n**Note:** You must keep a reference to channels created this way, or it will be closed."]
        #[inline]
        pub fn create_data_channel(&mut self, label: impl AsArg < GString >,) -> Option < Gd < crate::classes::WebRtcDataChannel > > {
            self.create_data_channel_ex(label,) . done()
        }
        #[doc = "Returns a new [`WebRTCDataChannel`][crate::classes::WebRtcDataChannel] (or `null` on failure) with given `label` and optionally configured via the `options` dictionary. This method can only be called when the connection is in state [`ConnectionState::NEW`][`crate::classes::web_rtc_peer_connection::ConnectionState::NEW`].\n\nThere are two ways to create a working data channel: either call [`create_data_channel`][`crate::classes::WebRtcPeerConnection::create_data_channel`] on only one of the peer and listen to `data_channel_received` on the other, or call [`create_data_channel`][`crate::classes::WebRtcPeerConnection::create_data_channel`] on both peers, with the same values, and the `\"negotiated\"` option set to `true`.\n\nValid `options` are:\n\n```gdscript\n{\n\t\"negotiated\": true, # When set to true (default off), means the channel is negotiated out of band. \"id\" must be set too. \"data_channel_received\" will not be called.\n\t\"id\": 1, # When \"negotiated\" is true this value must also be set to the same value on both peer.\n\n\t# Only one of maxRetransmits and maxPacketLifeTime can be specified, not both. They make the channel unreliable (but also better at real time).\n\t\"maxRetransmits\": 1, # Specify the maximum number of attempt the peer will make to retransmits packets if they are not acknowledged.\n\t\"maxPacketLifeTime\": 100, # Specify the maximum amount of time before giving up retransmitions of unacknowledged packets (in milliseconds).\n\t\"ordered\": true, # When in unreliable mode (i.e. either \"maxRetransmits\" or \"maxPacketLifetime\" is set), \"ordered\" (true by default) specify if packet ordering is to be enforced.\n\n\t\"protocol\": \"my-custom-protocol\", # A custom sub-protocol string for this channel.\n}\n```\n\n**Note:** You must keep a reference to channels created this way, or it will be closed."]
        #[inline]
        pub fn create_data_channel_ex < 'ex > (&'ex mut self, label: impl AsArg < GString > + 'ex,) -> ExCreateDataChannel < 'ex > {
            ExCreateDataChannel::new(self, label,)
        }
        #[doc = "Creates a new SDP offer to start a WebRTC connection with a remote peer. At least one [`WebRTCDataChannel`][crate::classes::WebRtcDataChannel] must have been created before calling this method.\n\nIf this functions returns [`Error::OK`][`crate::global::Error::OK`], `session_description_created` will be called when the session is ready to be sent."]
        pub fn create_offer(&mut self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7265usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "create_offer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the SDP description of the local peer. This should be called in response to `session_description_created`.\n\nAfter calling this function the peer will start emitting `ice_candidate_created` (unless an \\[enum Error] different from [`Error::OK`][`crate::global::Error::OK`] is returned)."]
        pub fn set_local_description(&mut self, type_: impl AsArg < GString >, sdp: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (type_.into_arg(), sdp.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7266usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "set_local_description", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sets the SDP description of the remote peer. This should be called with the values generated by a remote peer and received over the signaling server.\n\nIf `type` is `\"offer\"` the peer will emit `session_description_created` with the appropriate answer.\n\nIf `type` is `\"answer\"` the peer will start emitting `ice_candidate_created`."]
        pub fn set_remote_description(&mut self, type_: impl AsArg < GString >, sdp: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, CowArg < 'a1, GString >,);
            let args = (type_.into_arg(), sdp.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7267usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "set_remote_description", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Add an ice candidate generated by a remote peer (and received over the signaling server). See `ice_candidate_created`."]
        pub fn add_ice_candidate(&mut self, media: impl AsArg < GString >, index: i32, name: impl AsArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, GString >, i32, CowArg < 'a1, GString >,);
            let args = (media.into_arg(), index, name.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7268usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "add_ice_candidate", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Call this method frequently (e.g. in [`process`][`crate::classes::INode::process`] or [`physics_process`][`crate::classes::INode::physics_process`]) to properly receive signals."]
        pub fn poll(&mut self,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7269usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "poll", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Close the peer connection and all data channels associated with it.\n\n**Note:** You cannot reuse this object for a new connection unless you call [`initialize`][`crate::classes::WebRtcPeerConnection::initialize`]."]
        pub fn close(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7270usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "close", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the connection state."]
        pub fn get_connection_state(&self,) -> crate::classes::web_rtc_peer_connection::ConnectionState {
            type CallRet = crate::classes::web_rtc_peer_connection::ConnectionState;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7271usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "get_connection_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the ICE \\[enum GatheringState] of the connection. This lets you detect, for example, when collection of ICE candidates has finished."]
        pub fn get_gathering_state(&self,) -> crate::classes::web_rtc_peer_connection::GatheringState {
            type CallRet = crate::classes::web_rtc_peer_connection::GatheringState;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7272usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "get_gathering_state", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the signaling state on the local end of the connection while connecting or reconnecting to another peer."]
        pub fn get_signaling_state(&self,) -> crate::classes::web_rtc_peer_connection::SignalingState {
            type CallRet = crate::classes::web_rtc_peer_connection::SignalingState;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(7273usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "WebRtcPeerConnection", "get_signaling_state", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for WebRtcPeerConnection {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("WebRTCPeerConnection"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for WebRtcPeerConnection {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for WebRtcPeerConnection {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for WebRtcPeerConnection {
        
    }
    impl crate::obj::cap::GodotDefault for WebRtcPeerConnection {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for WebRtcPeerConnection {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for WebRtcPeerConnection {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`WebRtcPeerConnection`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_WebRtcPeerConnection__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::WebRtcPeerConnection > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`WebRtcPeerConnection::initialize_ex`][super::WebRtcPeerConnection::initialize_ex]."]
#[must_use]
pub struct ExInitialize < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::WebRtcPeerConnection, configuration: CowArg < 'ex, AnyDictionary >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExInitialize < 'ex > {
    fn new(surround_object: &'ex mut re_export::WebRtcPeerConnection,) -> Self {
        let configuration = AnyDictionary::new_untyped();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, configuration: CowArg::Owned(configuration),
        }
    }
    #[inline]
    pub fn configuration(self, configuration: &'ex AnyDictionary) -> Self {
        Self {
            configuration: CowArg::Borrowed(configuration), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, configuration,
        }
        = self;
        re_export::WebRtcPeerConnection::initialize_full(surround_object, configuration.cow_as_arg(),)
    }
}
#[doc = "Default-param extender for [`WebRtcPeerConnection::create_data_channel_ex`][super::WebRtcPeerConnection::create_data_channel_ex]."]
#[must_use]
pub struct ExCreateDataChannel < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::WebRtcPeerConnection, label: CowArg < 'ex, GString >, options: CowArg < 'ex, AnyDictionary >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExCreateDataChannel < 'ex > {
    fn new(surround_object: &'ex mut re_export::WebRtcPeerConnection, label: impl AsArg < GString > + 'ex,) -> Self {
        let options = AnyDictionary::new_untyped();
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, label: label.into_arg(), options: CowArg::Owned(options),
        }
    }
    #[inline]
    pub fn options(self, options: &'ex AnyDictionary) -> Self {
        Self {
            options: CowArg::Borrowed(options), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::WebRtcDataChannel > > {
        let Self {
            _phantom, surround_object, label, options,
        }
        = self;
        re_export::WebRtcPeerConnection::create_data_channel_full(surround_object, label, options.cow_as_arg(),)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ConnectionState {
    ord: i32
}
impl ConnectionState {
    #[doc(alias = "STATE_NEW")]
    #[doc = "Godot enumerator name: `STATE_NEW`"]
    pub const NEW: ConnectionState = ConnectionState {
        ord: 0i32
    };
    #[doc(alias = "STATE_CONNECTING")]
    #[doc = "Godot enumerator name: `STATE_CONNECTING`"]
    pub const CONNECTING: ConnectionState = ConnectionState {
        ord: 1i32
    };
    #[doc(alias = "STATE_CONNECTED")]
    #[doc = "Godot enumerator name: `STATE_CONNECTED`"]
    pub const CONNECTED: ConnectionState = ConnectionState {
        ord: 2i32
    };
    #[doc(alias = "STATE_DISCONNECTED")]
    #[doc = "Godot enumerator name: `STATE_DISCONNECTED`"]
    pub const DISCONNECTED: ConnectionState = ConnectionState {
        ord: 3i32
    };
    #[doc(alias = "STATE_FAILED")]
    #[doc = "Godot enumerator name: `STATE_FAILED`"]
    pub const FAILED: ConnectionState = ConnectionState {
        ord: 4i32
    };
    #[doc(alias = "STATE_CLOSED")]
    #[doc = "Godot enumerator name: `STATE_CLOSED`"]
    pub const CLOSED: ConnectionState = ConnectionState {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for ConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ConnectionState") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ConnectionState {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::NEW => "NEW", Self::CONNECTING => "CONNECTING", Self::CONNECTED => "CONNECTED", Self::DISCONNECTED => "DISCONNECTED", Self::FAILED => "FAILED", Self::CLOSED => "CLOSED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ConnectionState::NEW, ConnectionState::CONNECTING, ConnectionState::CONNECTED, ConnectionState::DISCONNECTED, ConnectionState::FAILED, ConnectionState::CLOSED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ConnectionState >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NEW", "STATE_NEW", ConnectionState::NEW), crate::meta::inspect::EnumConstant::new("CONNECTING", "STATE_CONNECTING", ConnectionState::CONNECTING), crate::meta::inspect::EnumConstant::new("CONNECTED", "STATE_CONNECTED", ConnectionState::CONNECTED), crate::meta::inspect::EnumConstant::new("DISCONNECTED", "STATE_DISCONNECTED", ConnectionState::DISCONNECTED), crate::meta::inspect::EnumConstant::new("FAILED", "STATE_FAILED", ConnectionState::FAILED), crate::meta::inspect::EnumConstant::new("CLOSED", "STATE_CLOSED", ConnectionState::CLOSED)]
        }
    }
}
impl crate::meta::GodotConvert for ConnectionState {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("State New", 0i64), EnumeratorShape::new_int("State Connecting", 1i64), EnumeratorShape::new_int("State Connected", 2i64), EnumeratorShape::new_int("State Disconnected", 3i64), EnumeratorShape::new_int("State Failed", 4i64), EnumeratorShape::new_int("State Closed", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("WebRTCPeerConnection.ConnectionState")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ConnectionState {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ConnectionState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ConnectionState {
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
impl crate::registry::property::Export for ConnectionState {
    
}
impl crate::meta::Element for ConnectionState {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct GatheringState {
    ord: i32
}
impl GatheringState {
    #[doc(alias = "GATHERING_STATE_NEW")]
    #[doc = "Godot enumerator name: `GATHERING_STATE_NEW`"]
    pub const NEW: GatheringState = GatheringState {
        ord: 0i32
    };
    #[doc(alias = "GATHERING_STATE_GATHERING")]
    #[doc = "Godot enumerator name: `GATHERING_STATE_GATHERING`"]
    pub const GATHERING: GatheringState = GatheringState {
        ord: 1i32
    };
    #[doc(alias = "GATHERING_STATE_COMPLETE")]
    #[doc = "Godot enumerator name: `GATHERING_STATE_COMPLETE`"]
    pub const COMPLETE: GatheringState = GatheringState {
        ord: 2i32
    };
    
}
impl std::fmt::Debug for GatheringState {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("GatheringState") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for GatheringState {
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
            Self::NEW => "NEW", Self::GATHERING => "GATHERING", Self::COMPLETE => "COMPLETE", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[GatheringState::NEW, GatheringState::GATHERING, GatheringState::COMPLETE]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < GatheringState >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NEW", "GATHERING_STATE_NEW", GatheringState::NEW), crate::meta::inspect::EnumConstant::new("GATHERING", "GATHERING_STATE_GATHERING", GatheringState::GATHERING), crate::meta::inspect::EnumConstant::new("COMPLETE", "GATHERING_STATE_COMPLETE", GatheringState::COMPLETE)]
        }
    }
}
impl crate::meta::GodotConvert for GatheringState {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Gathering State New", 0i64), EnumeratorShape::new_int("Gathering State Gathering", 1i64), EnumeratorShape::new_int("Gathering State Complete", 2i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("WebRTCPeerConnection.GatheringState")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for GatheringState {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for GatheringState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for GatheringState {
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
impl crate::registry::property::Export for GatheringState {
    
}
impl crate::meta::Element for GatheringState {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SignalingState {
    ord: i32
}
impl SignalingState {
    #[doc(alias = "SIGNALING_STATE_STABLE")]
    #[doc = "Godot enumerator name: `SIGNALING_STATE_STABLE`"]
    pub const STABLE: SignalingState = SignalingState {
        ord: 0i32
    };
    #[doc(alias = "SIGNALING_STATE_HAVE_LOCAL_OFFER")]
    #[doc = "Godot enumerator name: `SIGNALING_STATE_HAVE_LOCAL_OFFER`"]
    pub const HAVE_LOCAL_OFFER: SignalingState = SignalingState {
        ord: 1i32
    };
    #[doc(alias = "SIGNALING_STATE_HAVE_REMOTE_OFFER")]
    #[doc = "Godot enumerator name: `SIGNALING_STATE_HAVE_REMOTE_OFFER`"]
    pub const HAVE_REMOTE_OFFER: SignalingState = SignalingState {
        ord: 2i32
    };
    #[doc(alias = "SIGNALING_STATE_HAVE_LOCAL_PRANSWER")]
    #[doc = "Godot enumerator name: `SIGNALING_STATE_HAVE_LOCAL_PRANSWER`"]
    pub const HAVE_LOCAL_PRANSWER: SignalingState = SignalingState {
        ord: 3i32
    };
    #[doc(alias = "SIGNALING_STATE_HAVE_REMOTE_PRANSWER")]
    #[doc = "Godot enumerator name: `SIGNALING_STATE_HAVE_REMOTE_PRANSWER`"]
    pub const HAVE_REMOTE_PRANSWER: SignalingState = SignalingState {
        ord: 4i32
    };
    #[doc(alias = "SIGNALING_STATE_CLOSED")]
    #[doc = "Godot enumerator name: `SIGNALING_STATE_CLOSED`"]
    pub const CLOSED: SignalingState = SignalingState {
        ord: 5i32
    };
    
}
impl std::fmt::Debug for SignalingState {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("SignalingState") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for SignalingState {
    fn try_from_ord(ord: i32) -> Option < Self > {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => Some(Self {
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
            Self::STABLE => "STABLE", Self::HAVE_LOCAL_OFFER => "HAVE_LOCAL_OFFER", Self::HAVE_REMOTE_OFFER => "HAVE_REMOTE_OFFER", Self::HAVE_LOCAL_PRANSWER => "HAVE_LOCAL_PRANSWER", Self::HAVE_REMOTE_PRANSWER => "HAVE_REMOTE_PRANSWER", Self::CLOSED => "CLOSED", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[SignalingState::STABLE, SignalingState::HAVE_LOCAL_OFFER, SignalingState::HAVE_REMOTE_OFFER, SignalingState::HAVE_LOCAL_PRANSWER, SignalingState::HAVE_REMOTE_PRANSWER, SignalingState::CLOSED]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < SignalingState >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("STABLE", "SIGNALING_STATE_STABLE", SignalingState::STABLE), crate::meta::inspect::EnumConstant::new("HAVE_LOCAL_OFFER", "SIGNALING_STATE_HAVE_LOCAL_OFFER", SignalingState::HAVE_LOCAL_OFFER), crate::meta::inspect::EnumConstant::new("HAVE_REMOTE_OFFER", "SIGNALING_STATE_HAVE_REMOTE_OFFER", SignalingState::HAVE_REMOTE_OFFER), crate::meta::inspect::EnumConstant::new("HAVE_LOCAL_PRANSWER", "SIGNALING_STATE_HAVE_LOCAL_PRANSWER", SignalingState::HAVE_LOCAL_PRANSWER), crate::meta::inspect::EnumConstant::new("HAVE_REMOTE_PRANSWER", "SIGNALING_STATE_HAVE_REMOTE_PRANSWER", SignalingState::HAVE_REMOTE_PRANSWER), crate::meta::inspect::EnumConstant::new("CLOSED", "SIGNALING_STATE_CLOSED", SignalingState::CLOSED)]
        }
    }
}
impl crate::meta::GodotConvert for SignalingState {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Signaling State Stable", 0i64), EnumeratorShape::new_int("Signaling State Have Local Offer", 1i64), EnumeratorShape::new_int("Signaling State Have Remote Offer", 2i64), EnumeratorShape::new_int("Signaling State Have Local Pranswer", 3i64), EnumeratorShape::new_int("Signaling State Have Remote Pranswer", 4i64), EnumeratorShape::new_int("Signaling State Closed", 5i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("WebRTCPeerConnection.SignalingState")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for SignalingState {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for SignalingState {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for SignalingState {
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
impl crate::registry::property::Export for SignalingState {
    
}
impl crate::meta::Element for SignalingState {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::WebRtcPeerConnection;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`WebRtcPeerConnection`][crate::classes::WebRtcPeerConnection] class."]
    pub struct SignalsOfWebRtcPeerConnection < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfWebRtcPeerConnection < 'c, C > {
        #[doc = "Signature: `(type_: GString, sdp: GString)`"]
        pub fn session_description_created(&mut self) -> SigSessionDescriptionCreated < 'c, C > {
            SigSessionDescriptionCreated {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "session_description_created")
            }
        }
        #[doc = "Signature: `(media: GString, index: i64, name: GString)`"]
        pub fn ice_candidate_created(&mut self) -> SigIceCandidateCreated < 'c, C > {
            SigIceCandidateCreated {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "ice_candidate_created")
            }
        }
        #[doc = "Signature: `(channel: Gd<WebRtcDataChannel>)`"]
        pub fn data_channel_received(&mut self) -> SigDataChannelReceived < 'c, C > {
            SigDataChannelReceived {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "data_channel_received")
            }
        }
    }
    type TypedSigSessionDescriptionCreated < 'c, C > = TypedSignal < 'c, C, (GString, GString,) >;
    pub struct SigSessionDescriptionCreated < 'c, C: WithSignals > {
        typed: TypedSigSessionDescriptionCreated < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigSessionDescriptionCreated < 'c, C > {
        pub fn emit(&mut self, type_: GString, sdp: GString,) {
            self.typed.emit_tuple((type_, sdp,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigSessionDescriptionCreated < 'c, C > {
        type Target = TypedSigSessionDescriptionCreated < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigSessionDescriptionCreated < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigIceCandidateCreated < 'c, C > = TypedSignal < 'c, C, (GString, i64, GString,) >;
    pub struct SigIceCandidateCreated < 'c, C: WithSignals > {
        typed: TypedSigIceCandidateCreated < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigIceCandidateCreated < 'c, C > {
        pub fn emit(&mut self, media: GString, index: i64, name: GString,) {
            self.typed.emit_tuple((media, index, name,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigIceCandidateCreated < 'c, C > {
        type Target = TypedSigIceCandidateCreated < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigIceCandidateCreated < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigDataChannelReceived < 'c, C > = TypedSignal < 'c, C, (Gd < crate::classes::WebRtcDataChannel >,) >;
    pub struct SigDataChannelReceived < 'c, C: WithSignals > {
        typed: TypedSigDataChannelReceived < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigDataChannelReceived < 'c, C > {
        pub fn emit(&mut self, channel: Gd < crate::classes::WebRtcDataChannel >,) {
            self.typed.emit_tuple((channel,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigDataChannelReceived < 'c, C > {
        type Target = TypedSigDataChannelReceived < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigDataChannelReceived < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for WebRtcPeerConnection {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfWebRtcPeerConnection < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfWebRtcPeerConnection < 'c, C > {
        type Target = < < WebRtcPeerConnection as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = WebRtcPeerConnection;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfWebRtcPeerConnection < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = WebRtcPeerConnection;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
#![doc = "Sidecar module for class [`SceneMultiplayer`][crate::classes::SceneMultiplayer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SceneMultiplayer` enums](https://docs.godotengine.org/en/stable/classes/class_scenemultiplayer.html#enumerations).\n\n"]
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
    #[doc = "Godot class `SceneMultiplayer`.\n\nInherits [`MultiplayerApi`][crate::classes::MultiplayerApi].\n\nRelated symbols:\n\n* [`scene_multiplayer`][crate::classes::scene_multiplayer]: sidecar module with related enum/flag types\n* [`ISceneMultiplayer`][crate::classes::ISceneMultiplayer]: virtual methods\n* [`SignalsOfSceneMultiplayer`][crate::classes::scene_multiplayer::SignalsOfSceneMultiplayer]: signal collection\n\n\nSee also [Godot docs for `SceneMultiplayer`](https://docs.godotengine.org/en/stable/classes/class_scenemultiplayer.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`SceneMultiplayer::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nThis class is the default implementation of [`MultiplayerAPI`][crate::classes::MultiplayerApi], used to provide multiplayer functionalities in Godot Engine.\n\nThis implementation supports RPCs via [`rpc`][`crate::classes::Node::rpc`] and [`rpc_id`][`crate::classes::Node::rpc_id`] and requires [`rpc`][`crate::classes::MultiplayerApi::rpc`] to be passed a [`Node`][crate::classes::Node] (it will fail for other object types).\n\nThis implementation additionally provide [`SceneTree`][crate::classes::SceneTree] replication via the [`MultiplayerSpawner`][crate::classes::MultiplayerSpawner] and [`MultiplayerSynchronizer`][crate::classes::MultiplayerSynchronizer] nodes, and the [`SceneReplicationConfig`][crate::classes::SceneReplicationConfig] resource.\n\n**Note:** The high-level multiplayer API protocol is an implementation detail and isn't meant to be used by non-Godot servers. It may change without notice.\n\n**Note:** When exporting to Android, make sure to enable the `INTERNET` permission in the Android export preset before exporting the project or using one-click deploy. Otherwise, network communication of any kind will be blocked by Android."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct SceneMultiplayer {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`SceneMultiplayer`][crate::classes::SceneMultiplayer].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IMultiplayerApi`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `SceneMultiplayer` methods](https://docs.godotengine.org/en/stable/classes/class_scenemultiplayer.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ISceneMultiplayer: crate::obj::GodotClass < Base = SceneMultiplayer > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl SceneMultiplayer {
        pub fn set_root_path(&mut self, path: impl AsArg < NodePath >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, NodePath >,);
            let args = (path.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8543usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneMultiplayer", "set_root_path", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_root_path(&self,) -> NodePath {
            type CallRet = NodePath;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8544usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneMultiplayer", "get_root_path", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Clears the current SceneMultiplayer network state (you shouldn't call this unless you know what you are doing)."]
        pub fn clear(&mut self,) {
            type CallRet = ();
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8545usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneMultiplayer", "clear", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Disconnects the peer identified by `id`, removing it from the list of connected peers, and closing the underlying connection with it."]
        pub fn disconnect_peer(&mut self, id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8546usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneMultiplayer", "disconnect_peer", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the IDs of the peers currently trying to authenticate with this [`MultiplayerAPI`][crate::classes::MultiplayerApi]."]
        pub fn get_authenticating_peers(&self,) -> PackedInt32Array {
            type CallRet = PackedInt32Array;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8547usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneMultiplayer", "get_authenticating_peers", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sends the specified `data` to the remote peer identified by `id` as part of an authentication message. This can be used to authenticate peers, and control when `MultiplayerAPI.peer_connected` is emitted (and the remote peer accepted as one of the connected peers)."]
        pub fn send_auth(&mut self, id: i32, data: &PackedByteArray,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (i32, RefArg < 'a0, PackedByteArray >,);
            let args = (id, RefArg::new(data),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8548usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneMultiplayer", "send_auth", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Mark the authentication step as completed for the remote peer identified by `id`. The `MultiplayerAPI.peer_connected` signal will be emitted for this peer once the remote side also completes the authentication. No further authentication messages are expected to be received from this peer.\n\nIf a peer disconnects before completing authentication, either due to a network issue, the \\[member auth_timeout] expiring, or manually calling [`disconnect_peer`][`crate::classes::SceneMultiplayer::disconnect_peer`], the `peer_authentication_failed` signal will be emitted instead of `MultiplayerAPI.peer_disconnected`."]
        pub fn complete_auth(&mut self, id: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8549usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneMultiplayer", "complete_auth", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auth_callback(&mut self, callback: &Callable,) {
            type CallRet = ();
            type CallParams < 'a0, > = (RefArg < 'a0, Callable >,);
            let args = (RefArg::new(callback),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8550usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneMultiplayer", "set_auth_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_auth_callback(&self,) -> Callable {
            type CallRet = Callable;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8551usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneMultiplayer", "get_auth_callback", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_auth_timeout(&mut self, timeout: f64,) {
            type CallRet = ();
            type CallParams = (f64,);
            let args = (timeout,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8552usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneMultiplayer", "set_auth_timeout", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_auth_timeout(&self,) -> f64 {
            type CallRet = f64;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8553usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneMultiplayer", "get_auth_timeout", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_refuse_new_connections(&mut self, refuse: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (refuse,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8554usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneMultiplayer", "set_refuse_new_connections", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_refusing_new_connections(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8555usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneMultiplayer", "is_refusing_new_connections", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_allow_object_decoding(&mut self, enable: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enable,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8556usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneMultiplayer", "set_allow_object_decoding", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_object_decoding_allowed(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8557usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneMultiplayer", "is_object_decoding_allowed", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_server_relay_enabled(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8558usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneMultiplayer", "set_server_relay_enabled", Some(self.__validated_obj()), args,)
            }
        }
        pub fn is_server_relay_enabled(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8559usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneMultiplayer", "is_server_relay_enabled", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Sends the given raw `bytes` to a specific peer identified by `id` (see [`set_target_peer`][`crate::classes::MultiplayerPeer::set_target_peer`]). Default ID is `0`, i.e. broadcast to all peers."]
        pub(crate) fn send_bytes_full(&mut self, bytes: RefArg < PackedByteArray >, id: i32, mode: crate::classes::multiplayer_peer::TransferMode, channel: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (RefArg < 'a0, PackedByteArray >, i32, crate::classes::multiplayer_peer::TransferMode, i32,);
            let args = (bytes, id, mode, channel,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8560usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneMultiplayer", "send_bytes", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`send_bytes_ex`][Self::send_bytes_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Sends the given raw `bytes` to a specific peer identified by `id` (see [`set_target_peer`][`crate::classes::MultiplayerPeer::set_target_peer`]). Default ID is `0`, i.e. broadcast to all peers."]
        #[inline]
        pub fn send_bytes(&mut self, bytes: &PackedByteArray,) -> crate::global::Error {
            self.send_bytes_ex(bytes,) . done()
        }
        #[doc = "Sends the given raw `bytes` to a specific peer identified by `id` (see [`set_target_peer`][`crate::classes::MultiplayerPeer::set_target_peer`]). Default ID is `0`, i.e. broadcast to all peers."]
        #[inline]
        pub fn send_bytes_ex < 'ex > (&'ex mut self, bytes: &'ex PackedByteArray,) -> ExSendBytes < 'ex > {
            ExSendBytes::new(self, bytes,)
        }
        pub fn get_max_sync_packet_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8561usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneMultiplayer", "get_max_sync_packet_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_sync_packet_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8562usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneMultiplayer", "set_max_sync_packet_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn get_max_delta_packet_size(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8563usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneMultiplayer", "get_max_delta_packet_size", Some(self.__validated_obj()), args,)
            }
        }
        pub fn set_max_delta_packet_size(&mut self, size: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (size,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(8564usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "SceneMultiplayer", "set_max_delta_packet_size", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for SceneMultiplayer {
        type Base = crate::classes::MultiplayerApi;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("SceneMultiplayer"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for SceneMultiplayer {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::MultiplayerApi > for SceneMultiplayer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for SceneMultiplayer {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for SceneMultiplayer {
        
    }
    impl crate::obj::cap::GodotDefault for SceneMultiplayer {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for SceneMultiplayer {
        type Target = crate::classes::MultiplayerApi;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for SceneMultiplayer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`SceneMultiplayer`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_SceneMultiplayer__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::SceneMultiplayer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::MultiplayerApi > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`SceneMultiplayer::send_bytes_ex`][super::SceneMultiplayer::send_bytes_ex]."]
#[must_use]
pub struct ExSendBytes < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::SceneMultiplayer, bytes: CowArg < 'ex, PackedByteArray >, id: i32, mode: crate::classes::multiplayer_peer::TransferMode, channel: i32,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExSendBytes < 'ex > {
    fn new(surround_object: &'ex mut re_export::SceneMultiplayer, bytes: &'ex PackedByteArray,) -> Self {
        let id = 0i32;
        let mode = crate::obj::EngineEnum::from_ord(2);
        let channel = 0i32;
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, bytes: CowArg::Borrowed(bytes), id: id, mode: mode, channel: channel,
        }
    }
    #[inline]
    pub fn id(self, id: i32) -> Self {
        Self {
            id: id, .. self
        }
    }
    #[inline]
    pub fn mode(self, mode: crate::classes::multiplayer_peer::TransferMode) -> Self {
        Self {
            mode: mode, .. self
        }
    }
    #[inline]
    pub fn channel(self, channel: i32) -> Self {
        Self {
            channel: channel, .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, bytes, id, mode, channel,
        }
        = self;
        re_export::SceneMultiplayer::send_bytes_full(surround_object, bytes.cow_as_arg(), id, mode, channel,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::SceneMultiplayer;
    use crate::signal::TypedSignal;
    use super::*;
    #[doc = "A collection of signals for the [`SceneMultiplayer`][crate::classes::SceneMultiplayer] class."]
    pub struct SignalsOfSceneMultiplayer < 'c, C: WithSignals > {
        #[doc(hidden)]
        pub(crate) __internal_obj: Option < C::__SignalObj < 'c >>,
    }
    impl < 'c, C: WithSignals > SignalsOfSceneMultiplayer < 'c, C > {
        #[doc = "Signature: `(id: i64)`"]
        pub fn peer_authenticating(&mut self) -> SigPeerAuthenticating < 'c, C > {
            SigPeerAuthenticating {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "peer_authenticating")
            }
        }
        #[doc = "Signature: `(id: i64)`"]
        pub fn peer_authentication_failed(&mut self) -> SigPeerAuthenticationFailed < 'c, C > {
            SigPeerAuthenticationFailed {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "peer_authentication_failed")
            }
        }
        #[doc = "Signature: `(id: i64, packet: PackedByteArray)`"]
        pub fn peer_packet(&mut self) -> SigPeerPacket < 'c, C > {
            SigPeerPacket {
                typed: TypedSignal::__extract(&mut self.__internal_obj, "peer_packet")
            }
        }
    }
    type TypedSigPeerAuthenticating < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigPeerAuthenticating < 'c, C: WithSignals > {
        typed: TypedSigPeerAuthenticating < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPeerAuthenticating < 'c, C > {
        pub fn emit(&mut self, id: i64,) {
            self.typed.emit_tuple((id,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPeerAuthenticating < 'c, C > {
        type Target = TypedSigPeerAuthenticating < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPeerAuthenticating < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigPeerAuthenticationFailed < 'c, C > = TypedSignal < 'c, C, (i64,) >;
    pub struct SigPeerAuthenticationFailed < 'c, C: WithSignals > {
        typed: TypedSigPeerAuthenticationFailed < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPeerAuthenticationFailed < 'c, C > {
        pub fn emit(&mut self, id: i64,) {
            self.typed.emit_tuple((id,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPeerAuthenticationFailed < 'c, C > {
        type Target = TypedSigPeerAuthenticationFailed < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPeerAuthenticationFailed < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    type TypedSigPeerPacket < 'c, C > = TypedSignal < 'c, C, (i64, PackedByteArray,) >;
    pub struct SigPeerPacket < 'c, C: WithSignals > {
        typed: TypedSigPeerPacket < 'c, C >,
    }
    impl < 'c, C: WithSignals > SigPeerPacket < 'c, C > {
        pub fn emit(&mut self, id: i64, packet: PackedByteArray,) {
            self.typed.emit_tuple((id, packet,));
            
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SigPeerPacket < 'c, C > {
        type Target = TypedSigPeerPacket < 'c, C >;
        fn deref(&self) -> &Self::Target {
            &self.typed
        }
    }
    impl < C: WithSignals > std::ops::DerefMut for SigPeerPacket < '_, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.typed
        }
    }
    use crate::obj::WithSignals;
    impl WithSignals for SceneMultiplayer {
        type SignalCollection < 'c, C: WithSignals > = SignalsOfSceneMultiplayer < 'c, C >;
        type __SignalObj < 'c > = Gd < Self >;
        #[doc(hidden)]
        fn __signals_from_external(gd_ref: &Gd < Self >) -> Self::SignalCollection < '_, Self > {
            Self::SignalCollection {
                __internal_obj: Some(gd_ref.clone()),
            }
        }
    }
    impl < 'c, C: WithSignals > std::ops::Deref for SignalsOfSceneMultiplayer < 'c, C > {
        type Target = < < SceneMultiplayer as crate::obj::GodotClass > ::Base as WithSignals > ::SignalCollection < 'c, C >;
        fn deref(&self) -> &Self::Target {
            type Derived = SceneMultiplayer;
            crate::private::signal_collection_to_base::< C, Derived > (self)
        }
    }
    impl < 'c, C: WithSignals > std::ops::DerefMut for SignalsOfSceneMultiplayer < 'c, C > {
        fn deref_mut(&mut self) -> &mut Self::Target {
            type Derived = SceneMultiplayer;
            crate::private::signal_collection_to_base_mut::< C, Derived > (self)
        }
    }
}
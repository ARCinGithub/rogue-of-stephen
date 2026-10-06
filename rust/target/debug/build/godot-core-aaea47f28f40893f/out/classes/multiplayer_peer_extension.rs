#![doc = "Sidecar module for class [`MultiplayerPeerExtension`][crate::classes::MultiplayerPeerExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `MultiplayerPeerExtension` enums](https://docs.godotengine.org/en/stable/classes/class_multiplayerpeerextension.html#enumerations).\n\n"]
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
    #[doc = "Godot class `MultiplayerPeerExtension`.\n\nInherits [`MultiplayerPeer`][crate::classes::MultiplayerPeer].\n\nRelated symbols:\n\n* [`IMultiplayerPeerExtension`][crate::classes::IMultiplayerPeerExtension]: virtual methods\n\n\nSee also [Godot docs for `MultiplayerPeerExtension`](https://docs.godotengine.org/en/stable/classes/class_multiplayerpeerextension.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`MultiplayerPeerExtension::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nThis class is designed to be inherited from a GDExtension plugin to implement custom networking layers for the multiplayer API (such as WebRTC). All the methods below **must** be implemented to have a working custom multiplayer implementation. See also [`MultiplayerAPI`][crate::classes::MultiplayerApi]."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MultiplayerPeerExtension {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`MultiplayerPeerExtension`][crate::classes::MultiplayerPeerExtension].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: ~~`IMultiplayerPeer`~~ > ~~`IPacketPeer`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `MultiplayerPeerExtension` methods](https://docs.godotengine.org/en/stable/classes/class_multiplayerpeerextension.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IMultiplayerPeerExtension: crate::obj::GodotClass < Base = MultiplayerPeerExtension > + crate::private::You_forgot_the_attribute__godot_api {
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
        #[doc = "\n# Godot docs\nCalled when a packet needs to be received by the [`MultiplayerAPI`][crate::classes::MultiplayerApi], with `r_buffer_size` being the size of the binary `r_buffer` in bytes."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn get_packet_rawptr(&mut self, r_buffer: crate::meta::RawPtr < * mut crate::meta::RawPtr < * const u8 > >, r_buffer_size: crate::meta::RawPtr < * mut i32 >,) -> crate::global::Error {
            unimplemented !()
        }
        #[doc = "\n# Godot docs\nCalled when a packet needs to be sent by the [`MultiplayerAPI`][crate::classes::MultiplayerApi], with `p_buffer_size` being the size of the binary `p_buffer` in bytes."]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
        #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
        unsafe fn put_packet_rawptr(&mut self, p_buffer: crate::meta::RawPtr < * const u8 >, p_buffer_size: i32,) -> crate::global::Error {
            unimplemented !()
        }
        #[doc = "Called when the available packet count is internally requested by the [`MultiplayerAPI`][crate::classes::MultiplayerApi]."]
        fn get_available_packet_count(&self,) -> i32;
        #[doc = "Called when the maximum allowed packet size (in bytes) is requested by the [`MultiplayerAPI`][crate::classes::MultiplayerApi]."]
        fn get_max_packet_size(&self,) -> i32;
        #[doc = "Called when a packet needs to be received by the [`MultiplayerAPI`][crate::classes::MultiplayerApi], if [`get_packet_rawptr`][`crate::classes::IMultiplayerPeerExtension::get_packet_rawptr`] isn't implemented. Use this when extending this class via GDScript."]
        fn get_packet_script(&mut self,) -> PackedByteArray {
            unimplemented !()
        }
        #[doc = "Called when a packet needs to be sent by the [`MultiplayerAPI`][crate::classes::MultiplayerApi], if [`put_packet_rawptr`][`crate::classes::IMultiplayerPeerExtension::put_packet_rawptr`] isn't implemented. Use this when extending this class via GDScript."]
        fn put_packet_script(&mut self, p_buffer: PackedByteArray,) -> crate::global::Error {
            unimplemented !()
        }
        #[doc = "Called to get the channel over which the next available packet was received. See [`get_packet_channel`][`crate::classes::MultiplayerPeer::get_packet_channel`]."]
        fn get_packet_channel(&self,) -> i32;
        #[doc = "Called to get the transfer mode the remote peer used to send the next available packet. See [`get_packet_mode`][`crate::classes::MultiplayerPeer::get_packet_mode`]."]
        fn get_packet_mode(&self,) -> crate::classes::multiplayer_peer::TransferMode;
        #[doc = "Called when the channel to use is set for this [`MultiplayerPeer`][crate::classes::MultiplayerPeer] (see \\[member MultiplayerPeer.transfer_channel])."]
        fn set_transfer_channel(&mut self, p_channel: i32,);
        #[doc = "Called when the transfer channel to use is read on this [`MultiplayerPeer`][crate::classes::MultiplayerPeer] (see \\[member MultiplayerPeer.transfer_channel])."]
        fn get_transfer_channel(&self,) -> i32;
        #[doc = "Called when the transfer mode is set on this [`MultiplayerPeer`][crate::classes::MultiplayerPeer] (see \\[member MultiplayerPeer.transfer_mode])."]
        fn set_transfer_mode(&mut self, p_mode: crate::classes::multiplayer_peer::TransferMode,);
        #[doc = "Called when the transfer mode to use is read on this [`MultiplayerPeer`][crate::classes::MultiplayerPeer] (see \\[member MultiplayerPeer.transfer_mode])."]
        fn get_transfer_mode(&self,) -> crate::classes::multiplayer_peer::TransferMode;
        #[doc = "Called when the target peer to use is set for this [`MultiplayerPeer`][crate::classes::MultiplayerPeer] (see [`set_target_peer`][`crate::classes::MultiplayerPeer::set_target_peer`])."]
        fn set_target_peer(&mut self, p_peer: i32,);
        #[doc = "Called when the ID of the [`MultiplayerPeer`][crate::classes::MultiplayerPeer] who sent the most recent packet is requested (see [`get_packet_peer`][`crate::classes::MultiplayerPeer::get_packet_peer`])."]
        fn get_packet_peer(&self,) -> i32;
        #[doc = "Called when the \"is server\" status is requested on the [`MultiplayerAPI`][crate::classes::MultiplayerApi]. See [`is_server`][`crate::classes::MultiplayerApi::is_server`]."]
        fn is_server(&self,) -> bool;
        #[doc = "Called when the [`MultiplayerAPI`][crate::classes::MultiplayerApi] is polled. See [`poll`][`crate::classes::MultiplayerApi::poll`]."]
        fn poll(&mut self,);
        #[doc = "Called when the multiplayer peer should be immediately closed (see [`close`][`crate::classes::MultiplayerPeer::close`])."]
        fn close(&mut self,);
        #[doc = "Called when the connected `p_peer` should be forcibly disconnected (see [`disconnect_peer`][`crate::classes::MultiplayerPeer::disconnect_peer`])."]
        fn disconnect_peer(&mut self, p_peer: i32, p_force: bool,);
        #[doc = "Called when the unique ID of this [`MultiplayerPeer`][crate::classes::MultiplayerPeer] is requested (see [`get_unique_id`][`crate::classes::MultiplayerPeer::get_unique_id`]). The value must be between `1` and `2147483647`."]
        fn get_unique_id(&self,) -> i32;
        #[doc = "Called when the \"refuse new connections\" status is set on this [`MultiplayerPeer`][crate::classes::MultiplayerPeer] (see \\[member MultiplayerPeer.refuse_new_connections])."]
        fn set_refuse_new_connections(&mut self, p_enable: bool,) {
            unimplemented !()
        }
        #[doc = "Called when the \"refuse new connections\" status is requested on this [`MultiplayerPeer`][crate::classes::MultiplayerPeer] (see \\[member MultiplayerPeer.refuse_new_connections])."]
        fn is_refusing_new_connections(&self,) -> bool {
            unimplemented !()
        }
        #[doc = "Called to check if the server can act as a relay in the current configuration. See [`is_server_relay_supported`][`crate::classes::MultiplayerPeer::is_server_relay_supported`]."]
        fn is_server_relay_supported(&self,) -> bool {
            unimplemented !()
        }
        #[doc = "Called when the connection status is requested on the [`MultiplayerPeer`][crate::classes::MultiplayerPeer] (see [`get_connection_status`][`crate::classes::MultiplayerPeer::get_connection_status`])."]
        fn get_connection_status(&self,) -> crate::classes::multiplayer_peer::ConnectionStatus;
        
    }
    impl MultiplayerPeerExtension {
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
    impl crate::obj::GodotClass for MultiplayerPeerExtension {
        type Base = crate::classes::MultiplayerPeer;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("MultiplayerPeerExtension"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for MultiplayerPeerExtension {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::MultiplayerPeer > for MultiplayerPeerExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::PacketPeer > for MultiplayerPeerExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for MultiplayerPeerExtension {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for MultiplayerPeerExtension {
        
    }
    impl crate::obj::cap::GodotDefault for MultiplayerPeerExtension {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for MultiplayerPeerExtension {
        type Target = crate::classes::MultiplayerPeer;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for MultiplayerPeerExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`MultiplayerPeerExtension`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_MultiplayerPeerExtension__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::MultiplayerPeerExtension > for $Class {
                
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
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::MultiplayerPeerExtension;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::multiplayer_peer::SignalsOfMultiplayerPeer;
    impl WithSignals for MultiplayerPeerExtension {
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
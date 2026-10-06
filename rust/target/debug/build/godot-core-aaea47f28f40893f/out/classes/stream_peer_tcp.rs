#![doc = "Sidecar module for class [`StreamPeerTcp`][crate::classes::StreamPeerTcp].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `StreamPeerTCP` enums](https://docs.godotengine.org/en/stable/classes/class_streampeertcp.html#enumerations).\n\n"]
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
    #[doc = "Godot class `StreamPeerTCP`.\n\nInherits [`StreamPeerSocket`][crate::classes::StreamPeerSocket].\n\nRelated symbols:\n\n* [`stream_peer_tcp`][crate::classes::stream_peer_tcp]: sidecar module with related enum/flag types\n* [`IStreamPeerTcp`][crate::classes::IStreamPeerTcp]: virtual methods\n\n\nSee also [Godot docs for `StreamPeerTCP`](https://docs.godotengine.org/en/stable/classes/class_streampeertcp.html).\n\n"]
    #[doc = "# Construction\n\nThis class is reference-counted. You can create a new instance using [`StreamPeerTcp::new_gd()`][crate::obj::NewGd::new_gd].\n# Godot docs\nA stream peer that handles TCP connections. This object can be used to connect to TCP servers, or also is returned by a TCP server.\n\n**Note:** When exporting to Android, make sure to enable the `INTERNET` permission in the Android export preset before exporting the project or using one-click deploy. Otherwise, network communication of any kind will be blocked by Android."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct StreamPeerTcp {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    #[doc = "# Interface trait for class [`StreamPeerTcp`][crate::classes::StreamPeerTcp].\n\nFunctions in this trait represent constructors (`init`) or virtual method callbacks invoked by the engine.\n\n\n\n# Related symbols\n\nBase interfaces: [`IStreamPeerSocket`][crate::classes::IStreamPeerSocket] > ~~`IStreamPeer`~~ > [`IRefCounted`][crate::classes::IRefCounted] > [`IObject`][crate::classes::IObject].  \n(Strike-through means some intermediate Godot classes are marked final, and can thus not be inherited by GDExtension.)\n\n\n\nSee also [Godot docs for `StreamPeerTCP` methods](https://docs.godotengine.org/en/stable/classes/class_streampeertcp.html#methods)."]
    #[doc = ""]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait IStreamPeerTcp: crate::obj::GodotClass < Base = StreamPeerTcp > + crate::private::You_forgot_the_attribute__godot_api {
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
    impl StreamPeerTcp {
        #[doc = "Opens the TCP socket, and binds it to the specified local address.\n\nThis method is generally not needed, and only used to force the subsequent call to [`connect_to_host`][`crate::classes::StreamPeerTcp::connect_to_host`] to use the specified `host` and `port` as source address. This can be desired in some NAT punchthrough techniques, or when forcing the source network interface."]
        pub(crate) fn bind_full(&mut self, port: i32, host: CowArg < GString >,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (i32, CowArg < 'a0, GString >,);
            let args = (port, host,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11336usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeerTcp", "bind", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`bind_ex`][Self::bind_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Opens the TCP socket, and binds it to the specified local address.\n\nThis method is generally not needed, and only used to force the subsequent call to [`connect_to_host`][`crate::classes::StreamPeerTcp::connect_to_host`] to use the specified `host` and `port` as source address. This can be desired in some NAT punchthrough techniques, or when forcing the source network interface."]
        #[inline]
        pub fn bind(&mut self, port: i32,) -> crate::global::Error {
            self.bind_ex(port,) . done()
        }
        #[doc = "Opens the TCP socket, and binds it to the specified local address.\n\nThis method is generally not needed, and only used to force the subsequent call to [`connect_to_host`][`crate::classes::StreamPeerTcp::connect_to_host`] to use the specified `host` and `port` as source address. This can be desired in some NAT punchthrough techniques, or when forcing the source network interface."]
        #[inline]
        pub fn bind_ex < 'ex > (&'ex mut self, port: i32,) -> ExBind < 'ex > {
            ExBind::new(self, port,)
        }
        #[doc = "Connects to the specified `host:port` pair. A hostname will be resolved if valid. Returns [`Error::OK`][`crate::global::Error::OK`] on success."]
        pub fn connect_to_host(&mut self, host: impl AsArg < GString >, port: i32,) -> crate::global::Error {
            type CallRet = crate::global::Error;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, i32,);
            let args = (host.into_arg(), port,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11337usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeerTcp", "connect_to_host", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the IP of this peer."]
        pub fn get_connected_host(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11338usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeerTcp", "get_connected_host", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the port of this peer."]
        pub fn get_connected_port(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11339usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeerTcp", "get_connected_port", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the local port to which this peer is bound."]
        pub fn get_local_port(&self,) -> i32 {
            type CallRet = i32;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11340usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeerTcp", "get_local_port", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "If `enabled` is `true`, packets will be sent immediately. If `enabled` is `false` (the default), packet transfers will be delayed and combined using [Nagle's algorithm](https://en.wikipedia.org/wiki/Nagle%27s_algorithm).\n\n**Note:** It's recommended to leave this disabled for applications that send large packets or need to transfer a lot of data, as enabling this can decrease the total available bandwidth."]
        pub fn set_no_delay(&mut self, enabled: bool,) {
            type CallRet = ();
            type CallParams = (bool,);
            let args = (enabled,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11341usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "StreamPeerTcp", "set_no_delay", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for StreamPeerTcp {
        type Base = crate::classes::StreamPeerSocket;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("StreamPeerTCP"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for StreamPeerTcp {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::StreamPeerSocket > for StreamPeerTcp {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::StreamPeer > for StreamPeerTcp {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for StreamPeerTcp {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for StreamPeerTcp {
        
    }
    impl crate::obj::cap::GodotDefault for StreamPeerTcp {
        fn __godot_default() -> crate::obj::Gd < Self > {
            crate::classes::construct_engine_object::< Self > ()
        }
    }
    impl std::ops::Deref for StreamPeerTcp {
        type Target = crate::classes::StreamPeerSocket;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for StreamPeerTcp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = "The provided class must be a subclass of all the superclasses of [`StreamPeerTcp`]"]
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_StreamPeerTcp__ensure_class_exists {
        ($Class: ident) => {
            unsafe impl::godot::obj::Inherits < ::godot::classes::StreamPeerTcp > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::StreamPeerSocket > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::StreamPeer > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::RefCounted > for $Class {
                
            }
            unsafe impl::godot::obj::Inherits < ::godot::classes::Object > for $Class {
                
            }
        }
    }
}
#[doc = "Default-param extender for [`StreamPeerTcp::bind_ex`][super::StreamPeerTcp::bind_ex]."]
#[must_use]
pub struct ExBind < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::StreamPeerTcp, port: i32, host: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExBind < 'ex > {
    fn new(surround_object: &'ex mut re_export::StreamPeerTcp, port: i32,) -> Self {
        let host = GString::from("*");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, port: port, host: CowArg::Owned(host),
        }
    }
    #[inline]
    pub fn host(self, host: impl AsArg < GString > + 'ex) -> Self {
        Self {
            host: host.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> crate::global::Error {
        let Self {
            _phantom, surround_object, port, host,
        }
        = self;
        re_export::StreamPeerTcp::bind_full(surround_object, port, host,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::StreamPeerTcp;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for StreamPeerTcp {
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
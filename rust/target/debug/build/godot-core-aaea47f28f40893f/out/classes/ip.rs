#![doc = "Sidecar module for class [`Ip`][crate::classes::Ip].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `IP` enums](https://docs.godotengine.org/en/stable/classes/class_ip.html#enumerations).\n\n"]
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
    #[doc = "Godot class `IP`.\n\nInherits [`Object`][crate::classes::Object].\n\nRelated symbols:\n\n* [`ip`][crate::classes::ip]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `IP`](https://docs.godotengine.org/en/stable/classes/class_ip.html).\n\n"]
    #[doc = "# Singleton\n\nThis class is a singleton. You can get the one instance using [`Singleton::singleton()`][crate::obj::Singleton::singleton].\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nIP contains support functions for the Internet Protocol (IP). TCP/IP support is in different classes (see [`StreamPeerTCP`][crate::classes::StreamPeerTcp] and [`TCPServer`][crate::classes::TcpServer]). IP provides DNS hostname resolution support, both blocking and threaded."]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Ip {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl Ip {
        #[doc = "Returns a given hostname's IPv4 or IPv6 address when resolved (blocking-type method). The address type returned depends on the \\[enum Type] constant given as `ip_type`."]
        pub(crate) fn resolve_hostname_full(&mut self, host: CowArg < GString >, ip_type: crate::classes::ip::Type,) -> GString {
            type CallRet = GString;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::classes::ip::Type,);
            let args = (host, ip_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9753usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Ip", "resolve_hostname", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`resolve_hostname_ex`][Self::resolve_hostname_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Returns a given hostname's IPv4 or IPv6 address when resolved (blocking-type method). The address type returned depends on the \\[enum Type] constant given as `ip_type`."]
        #[inline]
        pub fn resolve_hostname(&mut self, host: impl AsArg < GString >,) -> GString {
            self.resolve_hostname_ex(host,) . done()
        }
        #[doc = "Returns a given hostname's IPv4 or IPv6 address when resolved (blocking-type method). The address type returned depends on the \\[enum Type] constant given as `ip_type`."]
        #[inline]
        pub fn resolve_hostname_ex < 'ex > (&'ex mut self, host: impl AsArg < GString > + 'ex,) -> ExResolveHostname < 'ex > {
            ExResolveHostname::new(self, host,)
        }
        #[doc = "Resolves a given hostname in a blocking way. Addresses are returned as an [`Array`][crate::builtin::Array] of IPv4 or IPv6 addresses depending on `ip_type`."]
        pub(crate) fn resolve_hostname_addresses_full(&mut self, host: CowArg < GString >, ip_type: crate::classes::ip::Type,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::classes::ip::Type,);
            let args = (host, ip_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9754usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Ip", "resolve_hostname_addresses", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`resolve_hostname_addresses_ex`][Self::resolve_hostname_addresses_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Resolves a given hostname in a blocking way. Addresses are returned as an [`Array`][crate::builtin::Array] of IPv4 or IPv6 addresses depending on `ip_type`."]
        #[inline]
        pub fn resolve_hostname_addresses(&mut self, host: impl AsArg < GString >,) -> PackedStringArray {
            self.resolve_hostname_addresses_ex(host,) . done()
        }
        #[doc = "Resolves a given hostname in a blocking way. Addresses are returned as an [`Array`][crate::builtin::Array] of IPv4 or IPv6 addresses depending on `ip_type`."]
        #[inline]
        pub fn resolve_hostname_addresses_ex < 'ex > (&'ex mut self, host: impl AsArg < GString > + 'ex,) -> ExResolveHostnameAddresses < 'ex > {
            ExResolveHostnameAddresses::new(self, host,)
        }
        #[doc = "Creates a queue item to resolve a hostname to an IPv4 or IPv6 address depending on the \\[enum Type] constant given as `ip_type`. Returns the queue ID if successful, or `RESOLVER_INVALID_ID` on error."]
        pub(crate) fn resolve_hostname_queue_item_full(&mut self, host: CowArg < GString >, ip_type: crate::classes::ip::Type,) -> i32 {
            type CallRet = i32;
            type CallParams < 'a0, > = (CowArg < 'a0, GString >, crate::classes::ip::Type,);
            let args = (host, ip_type,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9755usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Ip", "resolve_hostname_queue_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`resolve_hostname_queue_item_ex`][Self::resolve_hostname_queue_item_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a queue item to resolve a hostname to an IPv4 or IPv6 address depending on the \\[enum Type] constant given as `ip_type`. Returns the queue ID if successful, or `RESOLVER_INVALID_ID` on error."]
        #[inline]
        pub fn resolve_hostname_queue_item(&mut self, host: impl AsArg < GString >,) -> i32 {
            self.resolve_hostname_queue_item_ex(host,) . done()
        }
        #[doc = "Creates a queue item to resolve a hostname to an IPv4 or IPv6 address depending on the \\[enum Type] constant given as `ip_type`. Returns the queue ID if successful, or `RESOLVER_INVALID_ID` on error."]
        #[inline]
        pub fn resolve_hostname_queue_item_ex < 'ex > (&'ex mut self, host: impl AsArg < GString > + 'ex,) -> ExResolveHostnameQueueItem < 'ex > {
            ExResolveHostnameQueueItem::new(self, host,)
        }
        #[doc = "Returns a queued hostname's status as a \\[enum ResolverStatus] constant, given its queue `id`."]
        pub fn get_resolve_item_status(&self, id: i32,) -> crate::classes::ip::ResolverStatus {
            type CallRet = crate::classes::ip::ResolverStatus;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9756usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Ip", "get_resolve_item_status", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns a queued hostname's IP address, given its queue `id`. Returns an empty string on error or if resolution hasn't happened yet (see [`get_resolve_item_status`][`crate::classes::Ip::get_resolve_item_status`])."]
        pub fn get_resolve_item_address(&self, id: i32,) -> GString {
            type CallRet = GString;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9757usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Ip", "get_resolve_item_address", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns resolved addresses, or an empty array if an error happened or resolution didn't happen yet (see [`get_resolve_item_status`][`crate::classes::Ip::get_resolve_item_status`])."]
        pub fn get_resolve_item_addresses(&self, id: i32,) -> VarArray {
            type CallRet = VarArray;
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9758usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Ip", "get_resolve_item_addresses", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes a given item `id` from the queue. This should be used to free a queue after it has completed to enable more queries to happen."]
        pub fn erase_resolve_item(&mut self, id: i32,) {
            type CallRet = ();
            type CallParams = (i32,);
            let args = (id,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9759usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Ip", "erase_resolve_item", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns all the user's current IPv4 and IPv6 addresses as an array."]
        pub fn get_local_addresses(&self,) -> PackedStringArray {
            type CallRet = PackedStringArray;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9760usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Ip", "get_local_addresses", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns all network adapters as an array.\n\nEach adapter is a dictionary of the form:\n\n```gdscript\n{\n\t\"index\": \"1\", # Interface index.\n\t\"name\": \"eth0\", # Interface name.\n\t\"friendly\": \"Ethernet One\", # A friendly name (might be empty).\n\t\"addresses\": [\"192.168.1.101\"], # An array of IP addresses associated to this interface.\n}\n```"]
        pub fn get_local_interfaces(&self,) -> Array < VarDictionary > {
            type CallRet = Array < VarDictionary >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9761usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Ip", "get_local_interfaces", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Removes all of a `hostname`'s cached references. If no `hostname` is given, all cached IP addresses are removed."]
        pub(crate) fn clear_cache_full(&mut self, hostname: CowArg < GString >,) {
            type CallRet = ();
            type CallParams < 'a0, > = (CowArg < 'a0, GString >,);
            let args = (hostname,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(9762usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "Ip", "clear_cache", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "To set the default parameters, use [`clear_cache_ex`][Self::clear_cache_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Removes all of a `hostname`'s cached references. If no `hostname` is given, all cached IP addresses are removed."]
        #[inline]
        pub fn clear_cache(&mut self,) {
            self.clear_cache_ex() . done()
        }
        #[doc = "Removes all of a `hostname`'s cached references. If no `hostname` is given, all cached IP addresses are removed."]
        #[inline]
        pub fn clear_cache_ex < 'ex > (&'ex mut self,) -> ExClearCache < 'ex > {
            ExClearCache::new(self,)
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
        pub const RESOLVER_MAX_QUERIES: i32 = 256i32;
        pub const RESOLVER_INVALID_ID: i32 = - 1i32;
        
    }
    impl crate::obj::GodotClass for Ip {
        type Base = crate::classes::Object;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("IP"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for Ip {
        type Memory = crate::obj::bounds::MemManual;
        type DynMemory = crate::obj::bounds::MemManual;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for Ip {
        
    }
    impl crate::obj::Singleton for Ip {
        fn singleton() -> crate::obj::Gd < Self > {
            static CACHE: crate::classes::SingletonCache = crate::classes::SingletonCache::new();
            unsafe {
                crate::classes::cached_singleton::< Self > (&CACHE, || StringName::__cstr(c"IP"))
            }
        }
    }
    impl std::ops::Deref for Ip {
        type Target = crate::classes::Object;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for Ip {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_Ip__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `Ip` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`Ip::resolve_hostname_ex`][super::Ip::resolve_hostname_ex]."]
#[must_use]
pub struct ExResolveHostname < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Ip, host: CowArg < 'ex, GString >, ip_type: crate::classes::ip::Type,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExResolveHostname < 'ex > {
    fn new(surround_object: &'ex mut re_export::Ip, host: impl AsArg < GString > + 'ex,) -> Self {
        let ip_type = crate::obj::EngineEnum::from_ord(3);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, host: host.into_arg(), ip_type: ip_type,
        }
    }
    #[inline]
    pub fn ip_type(self, ip_type: crate::classes::ip::Type) -> Self {
        Self {
            ip_type: ip_type, .. self
        }
    }
    #[inline]
    pub fn done(self) -> GString {
        let Self {
            _phantom, surround_object, host, ip_type,
        }
        = self;
        re_export::Ip::resolve_hostname_full(surround_object, host, ip_type,)
    }
}
#[doc = "Default-param extender for [`Ip::resolve_hostname_addresses_ex`][super::Ip::resolve_hostname_addresses_ex]."]
#[must_use]
pub struct ExResolveHostnameAddresses < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Ip, host: CowArg < 'ex, GString >, ip_type: crate::classes::ip::Type,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExResolveHostnameAddresses < 'ex > {
    fn new(surround_object: &'ex mut re_export::Ip, host: impl AsArg < GString > + 'ex,) -> Self {
        let ip_type = crate::obj::EngineEnum::from_ord(3);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, host: host.into_arg(), ip_type: ip_type,
        }
    }
    #[inline]
    pub fn ip_type(self, ip_type: crate::classes::ip::Type) -> Self {
        Self {
            ip_type: ip_type, .. self
        }
    }
    #[inline]
    pub fn done(self) -> PackedStringArray {
        let Self {
            _phantom, surround_object, host, ip_type,
        }
        = self;
        re_export::Ip::resolve_hostname_addresses_full(surround_object, host, ip_type,)
    }
}
#[doc = "Default-param extender for [`Ip::resolve_hostname_queue_item_ex`][super::Ip::resolve_hostname_queue_item_ex]."]
#[must_use]
pub struct ExResolveHostnameQueueItem < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Ip, host: CowArg < 'ex, GString >, ip_type: crate::classes::ip::Type,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExResolveHostnameQueueItem < 'ex > {
    fn new(surround_object: &'ex mut re_export::Ip, host: impl AsArg < GString > + 'ex,) -> Self {
        let ip_type = crate::obj::EngineEnum::from_ord(3);
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, host: host.into_arg(), ip_type: ip_type,
        }
    }
    #[inline]
    pub fn ip_type(self, ip_type: crate::classes::ip::Type) -> Self {
        Self {
            ip_type: ip_type, .. self
        }
    }
    #[inline]
    pub fn done(self) -> i32 {
        let Self {
            _phantom, surround_object, host, ip_type,
        }
        = self;
        re_export::Ip::resolve_hostname_queue_item_full(surround_object, host, ip_type,)
    }
}
#[doc = "Default-param extender for [`Ip::clear_cache_ex`][super::Ip::clear_cache_ex]."]
#[must_use]
pub struct ExClearCache < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, surround_object: &'ex mut re_export::Ip, hostname: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExClearCache < 'ex > {
    fn new(surround_object: &'ex mut re_export::Ip,) -> Self {
        let hostname = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, surround_object: surround_object, hostname: CowArg::Owned(hostname),
        }
    }
    #[inline]
    pub fn hostname(self, hostname: impl AsArg < GString > + 'ex) -> Self {
        Self {
            hostname: hostname.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) {
        let Self {
            _phantom, surround_object, hostname,
        }
        = self;
        re_export::Ip::clear_cache_full(surround_object, hostname,)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ResolverStatus {
    ord: i32
}
impl ResolverStatus {
    #[doc(alias = "RESOLVER_STATUS_NONE")]
    #[doc = "Godot enumerator name: `RESOLVER_STATUS_NONE`"]
    pub const NONE: ResolverStatus = ResolverStatus {
        ord: 0i32
    };
    #[doc(alias = "RESOLVER_STATUS_WAITING")]
    #[doc = "Godot enumerator name: `RESOLVER_STATUS_WAITING`"]
    pub const WAITING: ResolverStatus = ResolverStatus {
        ord: 1i32
    };
    #[doc(alias = "RESOLVER_STATUS_DONE")]
    #[doc = "Godot enumerator name: `RESOLVER_STATUS_DONE`"]
    pub const DONE: ResolverStatus = ResolverStatus {
        ord: 2i32
    };
    #[doc(alias = "RESOLVER_STATUS_ERROR")]
    #[doc = "Godot enumerator name: `RESOLVER_STATUS_ERROR`"]
    pub const ERROR: ResolverStatus = ResolverStatus {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for ResolverStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("ResolverStatus") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for ResolverStatus {
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
            Self::NONE => "NONE", Self::WAITING => "WAITING", Self::DONE => "DONE", Self::ERROR => "ERROR", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[ResolverStatus::NONE, ResolverStatus::WAITING, ResolverStatus::DONE, ResolverStatus::ERROR]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < ResolverStatus >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "RESOLVER_STATUS_NONE", ResolverStatus::NONE), crate::meta::inspect::EnumConstant::new("WAITING", "RESOLVER_STATUS_WAITING", ResolverStatus::WAITING), crate::meta::inspect::EnumConstant::new("DONE", "RESOLVER_STATUS_DONE", ResolverStatus::DONE), crate::meta::inspect::EnumConstant::new("ERROR", "RESOLVER_STATUS_ERROR", ResolverStatus::ERROR)]
        }
    }
}
impl crate::meta::GodotConvert for ResolverStatus {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Resolver Status None", 0i64), EnumeratorShape::new_int("Resolver Status Waiting", 1i64), EnumeratorShape::new_int("Resolver Status Done", 2i64), EnumeratorShape::new_int("Resolver Status Error", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("IP.ResolverStatus")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for ResolverStatus {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for ResolverStatus {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for ResolverStatus {
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
impl crate::registry::property::Export for ResolverStatus {
    
}
impl crate::meta::Element for ResolverStatus {
    
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Type {
    ord: i32
}
impl Type {
    #[doc(alias = "TYPE_NONE")]
    #[doc = "Godot enumerator name: `TYPE_NONE`"]
    pub const NONE: Type = Type {
        ord: 0i32
    };
    #[doc(alias = "TYPE_IPV4")]
    #[doc = "Godot enumerator name: `TYPE_IPV4`"]
    pub const IPV4: Type = Type {
        ord: 1i32
    };
    #[doc(alias = "TYPE_IPV6")]
    #[doc = "Godot enumerator name: `TYPE_IPV6`"]
    pub const IPV6: Type = Type {
        ord: 2i32
    };
    #[doc(alias = "TYPE_ANY")]
    #[doc = "Godot enumerator name: `TYPE_ANY`"]
    pub const ANY: Type = Type {
        ord: 3i32
    };
    
}
impl std::fmt::Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter < '_ >) -> std::fmt::Result {
        use crate::obj::EngineEnum;
        let enumerator = self.as_str();
        if enumerator.is_empty() {
            f.debug_struct("Type") . field("ord", &self.ord) . finish() ?;
            return Ok(());
            
        }
        f.write_str(enumerator)
    }
}
impl crate::obj::EngineEnum for Type {
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
            Self::NONE => "NONE", Self::IPV4 => "IPV4", Self::IPV6 => "IPV6", Self::ANY => "ANY", _ => "",
        }
    }
    fn values() -> &'static[Self] {
        &[Type::NONE, Type::IPV4, Type::IPV6, Type::ANY]
    }
    fn all_constants() -> &'static[crate::meta::inspect::EnumConstant < Type >] {
        const {
            &[crate::meta::inspect::EnumConstant::new("NONE", "TYPE_NONE", Type::NONE), crate::meta::inspect::EnumConstant::new("IPV4", "TYPE_IPV4", Type::IPV4), crate::meta::inspect::EnumConstant::new("IPV6", "TYPE_IPV6", Type::IPV6), crate::meta::inspect::EnumConstant::new("ANY", "TYPE_ANY", Type::ANY)]
        }
    }
}
impl crate::meta::GodotConvert for Type {
    type Via = i32;
    fn godot_shape() -> crate::meta::shape::GodotShape {
        use crate::meta::shape::{
            EnumeratorShape, GodotShape
        };
        const ENUMERATORS: &[EnumeratorShape] = const {
            &[EnumeratorShape::new_int("Type None", 0i64), EnumeratorShape::new_int("Type Ipv4", 1i64), EnumeratorShape::new_int("Type Ipv6", 2i64), EnumeratorShape::new_int("Type Any", 3i64)]
        };
        GodotShape::Enum {
            variant_type: crate::meta::element_variant_type::< Self > (), enumerators: std::borrow::Cow::Borrowed(ENUMERATORS), godot_name: Some(std::borrow::Cow::Borrowed("IP.Type")), is_bitfield: false,
        }
    }
}
impl crate::meta::ToGodot for Type {
    type Pass = crate::meta::ByValue;
    fn to_godot(&self) -> Self::Via {
        < Self as crate::obj::EngineEnum > ::ord(* self)
    }
}
impl crate::meta::FromGodot for Type {
    fn try_from_godot(via: Self::Via) -> std::result::Result < Self, crate::meta::error::ConvertError > {
        < Self as crate::obj::EngineEnum > ::try_from_ord(via) . ok_or_else(|| crate::meta::error::FromGodotError::InvalidEnum.into_error(via as i64))
    }
}
impl crate::registry::property::Var for Type {
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
impl crate::registry::property::Export for Type {
    
}
impl crate::meta::Element for Type {
    
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::Ip;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for Ip {
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
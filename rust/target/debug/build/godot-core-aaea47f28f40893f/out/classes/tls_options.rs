#![doc = "Sidecar module for class [`TlsOptions`][crate::classes::TlsOptions].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TLSOptions` enums](https://docs.godotengine.org/en/stable/classes/class_tlsoptions.html#enumerations).\n\n"]
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
    #[doc = "Godot class `TLSOptions`.\n\nInherits [`RefCounted`][crate::classes::RefCounted].\n\nRelated symbols:\n\n* [`tls_options`][crate::classes::tls_options]: sidecar module with related enum/flag types\n\n\nSee also [Godot docs for `TLSOptions`](https://docs.godotengine.org/en/stable/classes/class_tlsoptions.html).\n\n"]
    #[doc = "# Not instantiable\n\nThis class cannot be constructed. Obtain `Gd<TlsOptions>` instances via Godot APIs.\n\n# Final class\n\nThis class is _final_, meaning you cannot inherit from it, and it comes without `I*` interface trait. It is still possible that other Godot classes inherit from it, but that is limited to the engine itself.\n# Godot docs\nTLSOptions abstracts the configuration options for the [`StreamPeerTLS`][crate::classes::StreamPeerTls] and [`PacketPeerDTLS`][crate::classes::PacketPeerDtls] classes.\n\nObjects of this class cannot be instantiated directly, and one of the static methods [`client`][`crate::classes::TlsOptions::client`], [`client_unsafe`][`crate::classes::TlsOptions::client_unsafe`], or [`server`][`crate::classes::TlsOptions::server`] should be used instead.\n\n\n```gdscript\n# Create a TLS client configuration which uses our custom trusted CA chain.\nvar client_trusted_cas = load(\"res://my_trusted_cas.crt\")\nvar client_tls_options = TLSOptions.client(client_trusted_cas)\n\n# Create a TLS server configuration.\nvar server_certs = load(\"res://my_server_cas.crt\")\nvar server_key = load(\"res://my_server_key.key\")\nvar server_tls_options = TLSOptions.server(server_key, server_certs)\n```\n"]
    #[derive(Debug)]
    #[repr(C)]
    pub struct TlsOptions {
        object_ptr: sys::GDExtensionObjectPtr, rtti: Option < crate::private::ObjectRtti >,
    }
    impl TlsOptions {
        #[doc = "Creates a TLS client configuration which validates certificates and their common names (fully qualified domain names).\n\nYou can specify a custom `trusted_chain` of certification authorities (the default CA list will be used if `null`), and optionally provide a `common_name_override` if you expect the certificate to have a common name other than the server FQDN.\n\n**Note:** On the Web platform, TLS verification is always enforced against the CA list of the web browser. This is considered a security feature."]
        pub(crate) fn client_full(trusted_chain: CowArg < Option < Gd < crate::classes::X509Certificate > > >, common_name_override: CowArg < GString >,) -> Option < Gd < crate::classes::TlsOptions > > {
            type CallRet = Option < Gd < crate::classes::TlsOptions > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::X509Certificate > > >, CowArg < 'a1, GString >,);
            let args = (trusted_chain, common_name_override,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11236usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TlsOptions", "client", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`client_ex`][Self::client_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates a TLS client configuration which validates certificates and their common names (fully qualified domain names).\n\nYou can specify a custom `trusted_chain` of certification authorities (the default CA list will be used if `null`), and optionally provide a `common_name_override` if you expect the certificate to have a common name other than the server FQDN.\n\n**Note:** On the Web platform, TLS verification is always enforced against the CA list of the web browser. This is considered a security feature."]
        #[inline]
        pub fn client() -> Option < Gd < crate::classes::TlsOptions > > {
            Self::client_ex() . done()
        }
        #[doc = "Creates a TLS client configuration which validates certificates and their common names (fully qualified domain names).\n\nYou can specify a custom `trusted_chain` of certification authorities (the default CA list will be used if `null`), and optionally provide a `common_name_override` if you expect the certificate to have a common name other than the server FQDN.\n\n**Note:** On the Web platform, TLS verification is always enforced against the CA list of the web browser. This is considered a security feature."]
        #[inline]
        pub fn client_ex < 'ex > () -> ExClient < 'ex > {
            ExClient::new()
        }
        #[doc = "Creates an **unsafe** TLS client configuration where certificate validation is optional. You can optionally provide a valid `trusted_chain`, but the common name of the certificates will never be checked. Using this configuration for purposes other than testing **is not recommended**.\n\n**Note:** On the Web platform, TLS verification is always enforced against the CA list of the web browser. This is considered a security feature."]
        pub(crate) fn client_unsafe_full(trusted_chain: CowArg < Option < Gd < crate::classes::X509Certificate > > >,) -> Option < Gd < crate::classes::TlsOptions > > {
            type CallRet = Option < Gd < crate::classes::TlsOptions > >;
            type CallParams < 'a0, > = (CowArg < 'a0, Option < Gd < crate::classes::X509Certificate > > >,);
            let args = (trusted_chain,);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11237usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TlsOptions", "client_unsafe", None, args,)
            }
        }
        #[doc = "To set the default parameters, use [`client_unsafe_ex`][Self::client_unsafe_ex] and its builder methods.  See [the book](https://godot-rust.github.io/book/godot-api/functions.html#default-parameters) for detailed usage instructions."]
        #[doc = "Creates an **unsafe** TLS client configuration where certificate validation is optional. You can optionally provide a valid `trusted_chain`, but the common name of the certificates will never be checked. Using this configuration for purposes other than testing **is not recommended**.\n\n**Note:** On the Web platform, TLS verification is always enforced against the CA list of the web browser. This is considered a security feature."]
        #[inline]
        pub fn client_unsafe() -> Option < Gd < crate::classes::TlsOptions > > {
            Self::client_unsafe_ex() . done()
        }
        #[doc = "Creates an **unsafe** TLS client configuration where certificate validation is optional. You can optionally provide a valid `trusted_chain`, but the common name of the certificates will never be checked. Using this configuration for purposes other than testing **is not recommended**.\n\n**Note:** On the Web platform, TLS verification is always enforced against the CA list of the web browser. This is considered a security feature."]
        #[inline]
        pub fn client_unsafe_ex < 'ex > () -> ExClientUnsafe < 'ex > {
            ExClientUnsafe::new()
        }
        #[doc = "Creates a TLS server configuration using the provided `key` and `certificate`.\n\n**Note:** The `certificate` should include the full certificate chain up to the signing CA (certificates file can be concatenated using a general purpose text editor)."]
        pub fn server(key: impl AsArg < Option < Gd < crate::classes::CryptoKey >> >, certificate: impl AsArg < Option < Gd < crate::classes::X509Certificate >> >,) -> Option < Gd < crate::classes::TlsOptions > > {
            type CallRet = Option < Gd < crate::classes::TlsOptions > >;
            type CallParams < 'a0, 'a1, > = (CowArg < 'a0, Option < Gd < crate::classes::CryptoKey > > >, CowArg < 'a1, Option < Gd < crate::classes::X509Certificate > > >,);
            let args = (key.into_arg(), certificate.into_arg(),);
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11238usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TlsOptions", "server", None, args,)
            }
        }
        #[doc = "Returns `true` if created with [`server`][`crate::classes::TlsOptions::server`], `false` otherwise."]
        pub fn is_server(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11239usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TlsOptions", "is_server", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns `true` if created with [`client_unsafe`][`crate::classes::TlsOptions::client_unsafe`], `false` otherwise."]
        pub fn is_unsafe_client(&self,) -> bool {
            type CallRet = bool;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11240usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TlsOptions", "is_unsafe_client", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the common name (domain name) override specified when creating with [`client`][`crate::classes::TlsOptions::client`]."]
        pub fn get_common_name_override(&self,) -> GString {
            type CallRet = GString;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11241usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TlsOptions", "get_common_name_override", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the CA [`X509Certificate`][crate::classes::X509Certificate] chain specified when creating with [`client`][`crate::classes::TlsOptions::client`] or [`client_unsafe`][`crate::classes::TlsOptions::client_unsafe`]."]
        pub fn get_trusted_ca_chain(&self,) -> Option < Gd < crate::classes::X509Certificate > > {
            type CallRet = Option < Gd < crate::classes::X509Certificate > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11242usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TlsOptions", "get_trusted_ca_chain", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`CryptoKey`][crate::classes::CryptoKey] specified when creating with [`server`][`crate::classes::TlsOptions::server`]."]
        pub fn get_private_key(&self,) -> Option < Gd < crate::classes::CryptoKey > > {
            type CallRet = Option < Gd < crate::classes::CryptoKey > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11243usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TlsOptions", "get_private_key", Some(self.__validated_obj()), args,)
            }
        }
        #[doc = "Returns the [`X509Certificate`][crate::classes::X509Certificate] specified when creating with [`server`][`crate::classes::TlsOptions::server`]."]
        pub fn get_own_certificate(&self,) -> Option < Gd < crate::classes::X509Certificate > > {
            type CallRet = Option < Gd < crate::classes::X509Certificate > >;
            type CallParams = ();
            let args = ();
            unsafe {
                let method_bind = sys::class_scene_api() . fptr_by_index(11244usize);
                Signature::< CallParams, CallRet > ::out_class_ptrcall(method_bind, "TlsOptions", "get_own_certificate", Some(self.__validated_obj()), args,)
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
    impl crate::obj::GodotClass for TlsOptions {
        type Base = crate::classes::RefCounted;
        fn class_id() -> ClassId {
            static CLASS_ID: std::sync::OnceLock < ClassId > = std::sync::OnceLock::new();
            let name: &'static ClassId = CLASS_ID.get_or_init(|| ClassId::__alloc_next_unicode("TLSOptions"));
            * name
        }
        const INIT_LEVEL: crate::init::InitLevel = crate::init::InitLevel::Scene;
        
    }
    unsafe impl crate::obj::Bounds for TlsOptions {
        type Memory = crate::obj::bounds::MemRefCounted;
        type DynMemory = crate::obj::bounds::MemRefCounted;
        type Declarer = crate::obj::bounds::DeclEngine;
        type Exportable = crate::obj::bounds::No;
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::RefCounted > for TlsOptions {
        
    }
    unsafe impl crate::obj::Inherits < crate::classes::Object > for TlsOptions {
        
    }
    impl std::ops::Deref for TlsOptions {
        type Target = crate::classes::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe {
                std::mem::transmute::< &Self, &Self::Target > (self)
            }
        }
    }
    impl std::ops::DerefMut for TlsOptions {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                std::mem::transmute::< &mut Self, &mut Self::Target > (self)
            }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules !inherit_from_TlsOptions__ensure_class_exists {
        ($Class: ident) => {
            compile_error !("Class `TlsOptions` is final, meaning it cannot be inherited in GDExtension or GDScript.");
            
        }
    }
}
#[doc = "Default-param extender for [`TlsOptions::client_ex`][super::TlsOptions::client_ex]."]
#[must_use]
pub struct ExClient < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, trusted_chain: CowArg < 'ex, Option < Gd < crate::classes::X509Certificate > > >, common_name_override: CowArg < 'ex, GString >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExClient < 'ex > {
    fn new() -> Self {
        let trusted_chain = Gd::null_arg();
        let common_name_override = GString::from("");
        Self {
            _phantom: std::marker::PhantomData, trusted_chain: trusted_chain.into_arg(), common_name_override: CowArg::Owned(common_name_override),
        }
    }
    #[inline]
    pub fn trusted_chain(self, trusted_chain: impl AsArg < Option < Gd < crate::classes::X509Certificate >> > + 'ex) -> Self {
        Self {
            trusted_chain: trusted_chain.into_arg(), .. self
        }
    }
    #[inline]
    pub fn common_name_override(self, common_name_override: impl AsArg < GString > + 'ex) -> Self {
        Self {
            common_name_override: common_name_override.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TlsOptions > > {
        let Self {
            _phantom, trusted_chain, common_name_override,
        }
        = self;
        re_export::TlsOptions::client_full(trusted_chain, common_name_override,)
    }
}
#[doc = "Default-param extender for [`TlsOptions::client_unsafe_ex`][super::TlsOptions::client_unsafe_ex]."]
#[must_use]
pub struct ExClientUnsafe < 'ex > {
    _phantom: std::marker::PhantomData < &'ex() >, trusted_chain: CowArg < 'ex, Option < Gd < crate::classes::X509Certificate > > >,
}
#[allow(clippy::wrong_self_convention, clippy::redundant_field_names, clippy::needless_update)]
impl < 'ex > ExClientUnsafe < 'ex > {
    fn new() -> Self {
        let trusted_chain = Gd::null_arg();
        Self {
            _phantom: std::marker::PhantomData, trusted_chain: trusted_chain.into_arg(),
        }
    }
    #[inline]
    pub fn trusted_chain(self, trusted_chain: impl AsArg < Option < Gd < crate::classes::X509Certificate >> > + 'ex) -> Self {
        Self {
            trusted_chain: trusted_chain.into_arg(), .. self
        }
    }
    #[inline]
    pub fn done(self) -> Option < Gd < crate::classes::TlsOptions > > {
        let Self {
            _phantom, trusted_chain,
        }
        = self;
        re_export::TlsOptions::client_unsafe_full(trusted_chain,)
    }
}
pub use signals::*;
mod signals {
    use crate::obj::{
        Gd, GodotClass
    };
    use super::re_export::TlsOptions;
    use crate::signal::TypedSignal;
    use super::*;
    use crate::obj::WithSignals;
    use crate::classes::object::SignalsOfObject;
    impl WithSignals for TlsOptions {
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